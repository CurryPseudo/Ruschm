use ruschm::{error::ErrorData, interpreter::*};

#[test]
fn bigint() {
    let mut interpreter = Interpreter::<f32>::new_with_stdlib();
    let r = interpreter.eval(include_str!("bigint.scm").chars());
    let r = r.unwrap_err().data;
    match r {
        ErrorData::Anyhow(e) => {
            assert_eq!(
                &e.to_string(),
                "integer 1231512145123 out of range when converted to i32"
            )
        }
        _ => unreachable!(),
    }
}
