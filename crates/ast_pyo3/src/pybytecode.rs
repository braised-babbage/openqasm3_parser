#[pyclass(frozen)]
#[derive(Clone)]
pub struct Bytecode {
    #[pyo3(get)]
    opcode: OpCode,
    #[pyo3(get)]
    operands: PyObject,
}

#[pyclass(frozen)]
#[derive(Clone)]
pub enum OpCode {
    GateCall,
    DeclareQreg,
//    DeclareQubit,
}
