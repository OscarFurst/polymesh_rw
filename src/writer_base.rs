use std::io::prelude::*;

pub fn write_single_data<T: std::fmt::Display>(
    data: &[T],
    file: &mut std::fs::File,
) -> std::io::Result<()> {
    writeln!(file, "{}", data.len())?;
    writeln!(file, "(")?;
    for d in data {
        writeln!(file, "{}", d)?;
    }
    writeln!(file, ")")?;
    Ok(())
}

/// Writes a vector of variable-width data to a file.
/// This kind of data is stored as a list of lists with specification of the length of the inner lists:
/// ```
///
pub fn write_multi_data<T: std::fmt::Display>(
    data: &[Vec<T>],
    file: &mut std::fs::File,
) -> std::io::Result<()> {
    writeln!(file, "{}", data.len())?;
    writeln!(file, "(")?;
    for d in data {
        writeln!(file, "{}", d.len())?;
        writeln!(file, "(")?;
        for dd in d {
            writeln!(file, "{}", dd)?;
        }
        writeln!(file, ")")?;
    }
    writeln!(file, ")")?;
    Ok(())
}

/// Writes a vector of fixed-width data to a file.
/// This kind of data is stored as a list of lists without specification of the length of the inner lists.
pub fn write_fixed_witdh_data<T, I>(data: &[I], file: &mut std::fs::File) -> std::io::Result<()>
where
    T: std::fmt::Display,
    for<'b> &'b I: IntoIterator<Item = &'b T>,
{
    writeln!(file, "{}", data.len())?;
    writeln!(file, "(")?;
    for d in data {
        write!(file, "(")?;
        for dd in d {
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
