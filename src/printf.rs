use crate::args::{Arg, ArgList};
use crate::locale::{Locale, C_LOCALE};
use crate::output::{wide_write, WideWrite};
use crate::wstr;

/// The sprintf function entry points. Prefer to use the macros below.
pub fn sprintf_locale<T: WideWrite>(target: &mut T, fmt: &wstr, locale: &Locale, args: &[Arg]) {
    let mut arglist = ArgList::new(args);
    let res = crate::parser::format(fmt, &mut arglist, wide_write(target, locale));
    if res.is_err() {
        panic!(
            "sprintf reported error \"{}\" with format string: {}",
            res.unwrap_err(),
            fmt
        );
    }
    if arglist.remaining() > 0 {
        panic!(
            "sprintf had {} unconsumed args for format string: {}",
            arglist.remaining(),
            fmt
        );
    }
}

pub fn sprintf_c_locale<T: WideWrite>(target: &mut T, fmt: &wstr, args: &[Arg]) {
    sprintf_locale(target, fmt, &C_LOCALE, args)
}

/// The basic entry point. Accepts a format string as a &wstr, and a list of arguments.
#[macro_export]
macro_rules! sprintf {
    // Variant which allows a string literal.
    (
        => $target:expr, // target string
        $fmt:literal, // format string
        $($arg:expr),* // arguments
        $(,)? // optional trailing comma
    ) => {
        {
            use $crate::args::ToArg;
            $crate::printf::sprintf_c_locale(
                $target,
                widestring::utf32str!($fmt),
                &[$($arg.to_arg()),*]
            )
        }
    };

    // Variant which allows a runtime format string, which must be of type &wstr.
    (
        => $target:expr, // target string
        $fmt:expr, // format string
        $($arg:expr),* // arguments
        $(,)? // optional trailing comma
    ) => {
        {
            use $crate::args::ToArg;
            $crate::printf::sprintf_c_locale(
                $target,
                $fmt,
                &[$($arg.to_arg()),*]
            )
        }
    };

    // Versions of the above which return a new string
    ($fmt:literal, $($arg:expr),* $(,)?) => {
        {
            let mut target = widestring::Utf32String::new();
            $crate::sprintf!(=> &mut target, $fmt, $($arg),*);
            target
        }
    };
    ($fmt:expr, $($arg:expr),* $(,)?) => {
        {
            let mut target = widestring::Utf32String::new();
            $crate::sprintf!(=> &mut target, $fmt, $($arg),*);
            target
        }
    };
}

#[cfg(test)]
mod tests {
    use widestring::utf32str;

    // Test basic sprintf with both literals and wide strings.
    #[test]
    fn test_sprintf() {
        assert_eq!(sprintf!("Hello, %s!", "world"), "Hello, world!");
        assert_eq!(sprintf!(utf32str!("Hello, %ls!"), "world"), "Hello, world!");
        assert_eq!(
            sprintf!(utf32str!("Hello, %ls!"), utf32str!("world")),
            "Hello, world!"
        );
    }
}
