//! RustPython serialization helpers — the backend's counterpart to the
//! pyo3 `pythonize`/pickle/dataclass machinery.
//!
//! JSON goes straight through `serde_json` (a plain `String`). Dict conversion
//! routes a value through `serde_json::Value` and a recursive
//! `Value` ⇄ `PyObjectRef` bridge, mirroring what `pythonize` does on the pyo3
//! side. Pickle reuses the shared `serde_pickle` bytes carried in a
//! `__pickle_state__` kwarg.

#![cfg(feature = "rustpython-backend")]

use rustpython_vm::{
    AsObject, PyObjectRef, PyResult, TryFromObject, VirtualMachine,
    builtins::{PyBytes, PyDict, PyFloat, PyInt, PyList, PyStr},
    function::FuncArgs,
};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

pub fn value_to_py(v: &Value, vm: &VirtualMachine) -> PyObjectRef {
    match v {
        Value::Null => vm.ctx.none(),
        Value::Bool(b) => vm.ctx.new_bool(*b).into(),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                vm.ctx.new_int(i).into()
            } else if let Some(u) = n.as_u64() {
                vm.ctx.new_int(u).into()
            } else {
                vm.ctx.new_float(n.as_f64().unwrap_or(0.0)).into()
            }
        }
        Value::String(s) => vm.ctx.new_str(s.as_str()).into(),
        Value::Array(arr) => {
            let items = arr.iter().map(|x| value_to_py(x, vm)).collect();
            vm.ctx.new_list(items).into()
        }
        Value::Object(map) => {
            let d = vm.ctx.new_dict();
            for (k, val) in map {
                let _ = d.set_item(k.as_str(), value_to_py(val, vm), vm);
            }
            d.into()
        }
    }
}

pub fn py_to_value(obj: &PyObjectRef, vm: &VirtualMachine) -> PyResult<Value> {
    if vm.is_none(obj) {
        return Ok(Value::Null);
    }
    if obj.fast_isinstance(vm.ctx.types.bool_type) {
        return Ok(Value::Bool(obj.clone().is_true(vm)?));
    }
    if obj.downcast_ref::<PyInt>().is_some() {
        let i = i64::try_from_object(vm, obj.clone())?;
        return Ok(Value::from(i));
    }
    if let Some(f) = obj.downcast_ref::<PyFloat>() {
        return Ok(Value::from(f.to_f64()));
    }
    if let Some(s) = obj.downcast_ref::<PyStr>() {
        return Ok(Value::String(s.to_string_lossy().into_owned()));
    }
    if let Some(list) = obj.downcast_ref::<PyList>() {
        let mut out = Vec::new();
        for item in list.borrow_vec().iter() {
            out.push(py_to_value(item, vm)?);
        }
        return Ok(Value::Array(out));
    }
    if let Some(dict) = obj.downcast_ref::<PyDict>() {
        let mut map = serde_json::Map::new();
        for (k, val) in dict {
            let key = k.str(vm)?.to_string_lossy().into_owned();
            map.insert(key, py_to_value(&val, vm)?);
        }
        return Ok(Value::Object(map));
    }
    Err(vm.new_type_error("cannot convert object to a serializable value".to_owned()))
}

pub fn to_json<T: Serialize>(v: &T, vm: &VirtualMachine) -> PyResult<String> {
    serde_json::to_string(v).map_err(|e| vm.new_value_error(format!("json serialize failed: {e}")))
}

pub fn from_json<T: DeserializeOwned>(s: &str, vm: &VirtualMachine) -> PyResult<T> {
    serde_json::from_str(s).map_err(|e| vm.new_value_error(format!("json deserialize failed: {e}")))
}

pub fn try_from_json<T: DeserializeOwned>(s: &str) -> Option<T> {
    serde_json::from_str(s).ok()
}

pub fn to_dict<T: Serialize>(v: &T, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
    let val = serde_json::to_value(v)
        .map_err(|e| vm.new_value_error(format!("serialize failed: {e}")))?;
    Ok(value_to_py(&val, vm))
}

pub fn from_dict<T: DeserializeOwned>(obj: &PyObjectRef, vm: &VirtualMachine) -> PyResult<T> {
    let val = py_to_value(obj, vm)?;
    serde_json::from_value(val).map_err(|e| vm.new_value_error(format!("deserialize failed: {e}")))
}

pub fn try_from_dict<T: DeserializeOwned>(obj: &PyObjectRef, vm: &VirtualMachine) -> Option<T> {
    py_to_value(obj, vm)
        .ok()
        .and_then(|val| serde_json::from_value(val).ok())
}

/// Build the `(args, kwargs)` 2-tuple returned by `__getnewargs_ex__`:
/// empty positional args plus a `__pickle_state__` kwarg carrying the
/// serde-pickle encoded bytes of `value`.
pub fn getnewargs_ex<T: Serialize>(value: &T, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
    let bytes = crate::pickle::pickle_encode_raw(value).map_err(|e| vm.new_value_error(e))?;
    let kwargs = vm.ctx.new_dict();
    kwargs.set_item("__pickle_state__", vm.ctx.new_bytes(bytes).into(), vm)?;
    let args = vm.ctx.new_tuple(vec![]);
    Ok(vm.ctx.new_tuple(vec![args.into(), kwargs.into()]).into())
}

/// Pull the `__pickle_state__` kwarg (if any) out of a constructor's args,
/// returning the raw serde-pickle bytes to be decoded by the caller.
pub fn take_pickle_state(args: &FuncArgs, vm: &VirtualMachine) -> PyResult<Option<Vec<u8>>> {
    match args.kwargs.get("__pickle_state__") {
        Some(obj) => {
            let bytes = obj
                .downcast_ref::<PyBytes>()
                .ok_or_else(|| vm.new_type_error("__pickle_state__ must be bytes".to_owned()))?;
            Ok(Some(bytes.as_bytes().to_vec()))
        }
        None => Ok(None),
    }
}

/// Build a `__dataclass_fields__` dict by constructing an ad-hoc dataclass via
/// the stdlib `dataclasses` module. Falls back to an empty dict if the module
/// is unavailable in the embedding.
pub fn dataclass_fields(names: &[&str], vm: &VirtualMachine) -> PyObjectRef {
    let build = || -> PyResult<PyObjectRef> {
        let dataclasses = vm.import("dataclasses", 0)?;
        let make_dc = dataclasses.get_attr("make_dataclass", vm)?;
        let name: PyObjectRef = vm.ctx.new_str("_GeomanpyDataclassShape").into();
        let fields: Vec<PyObjectRef> = names.iter().map(|n| vm.ctx.new_str(*n).into()).collect();
        let fields_list: PyObjectRef = vm.ctx.new_list(fields).into();
        let dummy = make_dc.call(vec![name, fields_list], vm)?;
        dummy.get_attr("__dataclass_fields__", vm)
    };
    build().unwrap_or_else(|_| vm.ctx.new_dict().into())
}
