from pathlib import Path
p=Path('01_core/src/compiler/layout/tiling.rs');s=p.read_text();a=s.index('    // Whole pitches change cell indices,');b=s.index('    let determinant = ',a)
s=s[:a]+'''    let placement = periodic_placement(
        placement_transform(tiling.relative, pos, offset_x, offset_y, angle),
        pitch_x,
        pitch_y,
    )?;
'''+s[b:]
s=s.replace('''    TransformMatrix::rotate(angle)
        .concat(&TransformMatrix::translate(offset_x + basis_x, offset_y + basis_y))
''','''    TransformMatrix::translate(offset_x + basis_x, offset_y + basis_y)
        .concat(&TransformMatrix::rotate(angle))
''')
a=s.index('\n#[cfg(test)]');s=s[:a]+'''
// Whole periods are interchangeable only in the rotated lattice basis.
// Keeping the phase bounded avoids integer saturation at large offsets.
fn periodic_placement(
    mut matrix: TransformMatrix,
    pitch_x: f64,
    pitch_y: f64,
) -> Option<TransformMatrix> {
    let determinant = matrix.a * matrix.d - matrix.b * matrix.c;
    let x = (matrix.d * matrix.tx - matrix.c * matrix.ty) / determinant;
    let y = (matrix.a * matrix.ty - matrix.b * matrix.tx) / determinant;
    if !x.is_finite() || !y.is_finite() {
        return None;
    }
    let x = x.rem_euclid(pitch_x);
    let y = y.rem_euclid(pitch_y);
    matrix.tx = matrix.a * x + matrix.c * y;
    matrix.ty = matrix.b * x + matrix.d * y;
    Some(matrix)
}
'''+s[a:];p.write_text(s)
p=Path('01_core/src/entities/tiling.rs');s=p.read_text();s=s.replace('/// Rotação horária aplicada depois do offset.','/// Rotação horária da grade em torno do ponto offset.');p.write_text(s)
