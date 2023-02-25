use std::ptr::null_mut;

pub struct Object {
    internal: basiclu_sys::basiclu_object,
}

impl Drop for Object {
    fn drop(&mut self) {
        unsafe { basiclu_sys::basiclu_obj_free(&mut self.internal) }
    }
}

impl Object {
    pub fn new() -> Self {
        Self {
            internal: basiclu_sys::basiclu_object {
                istore: null_mut(),
                xstore: null_mut(),
                Li: null_mut(),
                Ui: null_mut(),
                Wi: null_mut(),
                Lx: null_mut(),
                Ux: null_mut(),
                Wx: null_mut(),
                lhs: null_mut(),
                ilhs: null_mut(),
                nzlhs: 0 as basiclu_sys::lu_int,
                realloc_factor: 1.2,
            },
        }
    }

    pub fn initialize(&mut self, m: usize) {
        let _ = unsafe {
            basiclu_sys::basiclu_obj_initialize(&mut self.internal, m as basiclu_sys::lu_int)
        };
    }

    pub fn factorize(&mut self, b_begin: &[i64], b_end: &[i64], b_i: &[i64], b_x: &[f64]) {
        let _ = unsafe {
            basiclu_sys::basiclu_obj_factorize(
                &mut self.internal,
                b_begin.as_ptr(),
                b_end.as_ptr(),
                b_i.as_ptr(),
                b_x.as_ptr(),
            )
        };
    }

    pub fn solve(&mut self, rhs: &[f64], lhs: &mut [f64], transpose: bool) {
        let _ = unsafe {
            basiclu_sys::basiclu_obj_solve_dense(
                &mut self.internal,
                rhs.as_ptr(),
                lhs.as_mut_ptr(),
                if transpose { 'T' } else { 'N' } as i8,
            )
        };
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn simple_test() {
        // A = [
        //   [2.10                               0.14 0.09     ]
        //   [     1.10           0.06                     0.03]
        //   [          1.70                               0.04]
        //   [               1.00           0.32 0.19 0.32 0.44]
        //   [     0.06           1.60                         ]
        //   [                         2.20                    ]
        //   [               0.32           1.90           0.43]
        //   [0.14           0.19                1.10 0.22     ]
        //   [0.09           0.32                0.22 2.40     ]
        //   [     0.03 0.04 0.44           0.43           3.20]
        // ]
        let n = 10;
        let arow = vec![
            0, 7, 8, 1, 4, 9, 2, 9, 3, 6, 7, 8, 9, 1, 4, 5, 3, 6, 9, 0, 3, 7, 8, 0, 3, 7, 8, 1, 2,
            3, 6, 9,
        ];
        let acolst = vec![0, 3, 6, 8, 13, 15, 16, 19, 23, 27, 32];
        let a = vec![
            2.1, 0.14, 0.09, 1.1, 0.06, 0.03, 1.7, 0.04, 1.0, 0.32, 0.19, 0.32, 0.44, 0.06, 1.6,
            2.2, 0.32, 1.9, 0.43, 0.14, 0.19, 1.1, 0.22, 0.09, 0.32, 0.22, 2.4, 0.03, 0.04, 0.44,
            0.43, 3.2,
        ];

        let b = vec![
            0.403, 0.28, 0.55, 1.504, 0.812, 1.32, 1.888, 1.168, 2.473, 3.695,
        ];

        let mut lu = crate::Object::new();
        {
            lu.initialize(n);
            lu.factorize(&acolst, &acolst[1..], &arow, &a);

            let mut x = b.clone();

            lu.solve(&b, &mut x, false);

            println!("{:?}", x);
        }
    }
}
