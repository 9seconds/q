// GentlePanic trait (for more user-friendly exit message)


use std::io;
use std::io::Write;
use std::process;


pub trait GentlePanic<T> {
    fn get_or_die_with(self, exit_code: i32, content: &str) -> T;
}


impl<T> GentlePanic<T> for Option<T> {
    #[inline]
    fn get_or_die_with(self, exit_code: i32, message: &str) -> T {
        match self {
            Some(content) => content,
            None => exit_with_message(exit_code, message, "")
        }
    }
}


impl<T, E> GentlePanic<T> for Result<T, E> where E: AsRef<str> {
    #[inline]
    fn get_or_die_with(self, exit_code: i32, message: &str) -> T {
        match self {
            Ok(content) => content,
            Err(error) => exit_with_message(exit_code, message, error.as_ref())
        }
    }
}


#[inline]
#[allow(unreachable_code)]
fn exit_with_message<T>(exit_code: i32, message: &str, error: &str) -> T {
    let stderr_stream = io::stderr();
    let mut stderr_lock = stderr_stream.lock();

    if error == "" {
        let _ = writeln!(&mut stderr_lock, "{}", message);
    } else {
        let _ = writeln!(&mut stderr_lock, "{}: {}", message, error);
    };
    let _ = stderr_lock.flush();

    process::exit(exit_code);

    unreachable!()
}
