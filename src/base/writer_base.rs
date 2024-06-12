pub fn write_single_data<T: std::fmt::Display>(
    data: &[T],
    file: &mut std::fmt::Formatter,
) -> std::fmt::Result {
    writeln!(file, "{}", data.len())?;
    writeln!(file, "(")?;
    for d in data {
        writeln!(file, "{}", d)?;
    }
    writeln!(file, ")")?;
    Ok(())
}

/// Writes a vector of data to a file as a space-separated list.
pub fn write_vector_content<T: std::fmt::Display>(
    data: &[T],
    file: &mut std::fmt::Formatter,
) -> std::fmt::Result {
    let mut first = true;
    for d in data {
        if first {
            first = false;
        } else {
            write!(file, " ")?;
        }
        write!(file, "{}", d)?;
    }
    Ok(())
}

/// Writes a vector of variable-width data to a file.
/// This kind of data is stored as a list of lists with specification of the length of the inner lists:
/// ```text
/// 3
/// (
/// 3(1 2 3)
/// 2(4 5)
/// 4(6 7 8 9)
/// )
/// ```
pub fn write_multi_data<T: std::fmt::Display>(
    data: &[Vec<T>],
    file: &mut std::fmt::Formatter,
) -> std::fmt::Result {
    writeln!(file, "{}", data.len())?;
    writeln!(file, "(")?;
    for d in data {
        write!(file, "{}", d.len())?;
        write!(file, "(")?;
        let mut first = true;
        for dd in d {
            if first {
                first = false;
            } else {
                write!(file, " ")?;
            }
            write!(file, "{}", dd)?;
        }
        writeln!(file, ")")?;
    }
    writeln!(file, ")")?;
    Ok(())
}

/// Writes a vector of fixed-width data to a file.
/// This kind of data is stored as a list of lists without specification of the length of the inner lists:
/// ```text
/// 4
/// (
/// (1 2 3)
/// (4 5 6)
/// (7 8 9)
/// (10 11 12)
/// )
/// ```
pub fn write_fixed_witdh_data<T, I>(data: &[I], file: &mut std::fmt::Formatter) -> std::fmt::Result
where
    T: std::fmt::Display,
    for<'b> &'b I: IntoIterator<Item = &'b T>,
{
    writeln!(file, "{}", data.len())?;
    writeln!(file, "(")?;
    for d in data {
        write!(file, "(")?;
        let mut first = true;
        for dd in d {
            if first {
                first = false;
            } else {
                write!(file, " ")?;
            }
            write!(file, "{}", dd)?;
        }
        writeln!(file, ")")?;
    }
    writeln!(file, ")")?;
    Ok(())
}

pub fn bool_as_num(b: bool) -> usize {
    if b {
        1
    } else {
        0
    }
}
