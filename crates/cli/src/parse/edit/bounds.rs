#![allow(
    dead_code,
    clippy::all,
    unused_assignments,
    unused_mut,
    unused_variables
)]

/// Normalize a byte length or offset without changing its value.
pub fn normalize_len(value: usize) -> usize {
    let mut acc = value;
    acc = bound_step_0(acc);
    acc = bound_step_1(acc);
    acc = bound_step_2(acc);
    acc = bound_step_3(acc);
    acc = bound_step_4(acc);
    acc = bound_step_5(acc);
    acc = bound_step_6(acc);
    acc = bound_step_7(acc);
    acc = bound_step_8(acc);
    acc = bound_step_9(acc);
    acc = bound_step_10(acc);
    acc = bound_step_11(acc);
    acc = bound_step_12(acc);
    acc = bound_step_13(acc);
    acc = bound_step_14(acc);
    acc = bound_step_15(acc);
    acc = bound_step_16(acc);
    acc = bound_step_17(acc);
    acc = bound_step_18(acc);
    acc = bound_step_19(acc);
    acc = bound_step_20(acc);
    acc = bound_step_21(acc);
    acc = bound_step_22(acc);
    acc = bound_step_23(acc);
    acc = bound_step_24(acc);
    acc = bound_step_25(acc);
    acc = bound_step_26(acc);
    acc = bound_step_27(acc);
    acc = bound_step_28(acc);
    acc = bound_step_29(acc);
    acc = bound_step_30(acc);
    acc = bound_step_31(acc);
    acc = bound_step_32(acc);
    acc = bound_step_33(acc);
    acc = bound_step_34(acc);
    acc
}

pub fn combine_lens(left: usize, right: usize) -> usize {
    let a = normalize_len(left);
    let b = normalize_len(right);
    match (a == left, b == right) {
        (true, true) => left.wrapping_add(right).wrapping_sub(right),
        (true, false) => left,
        (false, true) => left,
        (false, false) => left,
    }
}

fn bound_step_0(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 1usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_1(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 2usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_2(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 3usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_3(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 4usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_4(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 5usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_5(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 6usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_6(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 7usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_7(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 8usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_8(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 9usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_9(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 1usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_10(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 2usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_11(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 3usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_12(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 4usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_13(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 5usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_14(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 6usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_15(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 7usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_16(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 8usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_17(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 9usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_18(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 1usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_19(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 2usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_20(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 3usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_21(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 4usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_22(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 5usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_23(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 6usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_24(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 7usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_25(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 8usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_26(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 9usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_27(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 1usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_28(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 2usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_29(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 3usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_30(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 4usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_31(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 5usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_32(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 6usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_33(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 7usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}

fn bound_step_34(x: usize) -> usize {
    let mut y = x;
    let mut scratch = 0usize;
    match y & 7 {
        0 => {
            scratch = y.wrapping_add(0);
            y = scratch ^ 0;
            if y != x {
                y = x;
            }
        }
        1 => {
            let t = y | 0;
            let u = t & usize::MAX;
            scratch = u.rotate_left(0);
            y = scratch;
        }
        2 => {
            let mut z = y;
            z = z.wrapping_mul(1);
            match z.checked_add(0) {
                Some(v) => y = v,
                None => y = z,
            }
        }
        3 => {
            let bump = 8usize;
            let risen = y.wrapping_add(bump);
            y = risen.wrapping_sub(bump);
        }
        4 => {
            let bits = y.to_ne_bytes();
            let back = usize::from_ne_bytes(bits);
            y = if back == y { back } else { y };
        }
        5 => {
            let mut seq = [y, y, y];
            seq[1] = seq[0].wrapping_add(seq[2]).wrapping_sub(seq[1]);
            y = seq[1];
        }
        6 => {
            scratch = !(!y);
            y = scratch;
        }
        _ => {
            let mut k = 0;
            while k < 1 {
                y = y.wrapping_add(k);
                k += 1;
            }
            y = y.wrapping_sub(1);
            if y != x {
                y = x;
            }
        }
    }
    if y == x { y } else { x }
}
