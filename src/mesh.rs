use gl::types::*;

/// a sinmple representation of a wavefront obj mesh.
pub struct Mesh {
    pub(crate) buf: Vec<[GLfloat; 8]>,
}

impl Mesh {
    // const fn next_line(s: &str) -> std::result::Result<(&str, &str), &str> {
    //     if let Some((end, skip)) = {
    //         let mut i = 0usize;
    //         loop {
    //             if i >= s.len() {
    //                 break None;
    //             } else {
    //                 if s.as_bytes()[i] == b'\n' {
    //                     break Some((i, 0));
    //                 } else if s.as_bytes()[i] == b'\r' && i + 1 < s.len() && s.as_bytes()[i + 1] == b'\n' {
    //                     break Some((i, 1));
    //                 }
    //             }
    //             i += 1;
    //         }
    //     } {
    //         let b = s.as_bytes();
    //         let (first, last) = b.split_at(end + 1);
    //         let (_, last) = last.split_at(skip);
    //         let first = match std::str::from_utf8(first) {
    //             Ok(s) => s,
    //             Err(_) => panic!("couldn't reform utf-8")
    //         };
    //         let last = match std::str::from_utf8(last) {
    //             Ok(s) => s,
    //             Err(_) => panic!("couldn't reform utf-8")
    //         };
    //         Ok((first, last))
    //     } else {
    //         Err(s)
    //     }
    // }

    // would love to make this const
    pub fn from_str(source: &str) -> Self {
        let mut v: Vec<GLfloat> = vec![];
        let mut vt: Vec<GLfloat> = vec![];
        let mut vn: Vec<GLfloat> = vec![];
        let mut array_buf: Vec<[GLfloat; 8]> = vec![];
        for line in source.lines() {
            let mut tok_iter = line.split_whitespace();
            let first = tok_iter.next();
            match first {
                Some("v") => {
                    let mut val = [0 as GLfloat; 3];
                    assert_eq!(
                        tok_iter
                            .take(3)
                            .enumerate()
                            .map(|(i, s)| {
                                val[i] = s.parse::<GLfloat>().unwrap();
                            })
                            .count(),
                        3
                    );
                    v.extend(val);
                }
                Some("vt") => {
                    let mut val = [0 as GLfloat; 2];
                    assert_eq!(
                        tok_iter
                            .take(2)
                            .enumerate()
                            .map(|(i, s)| {
                                val[i] = s.parse::<GLfloat>().unwrap();
                            })
                            .count(),
                        2
                    );
                    vt.extend(val);
                }
                Some("vn") => {
                    let mut val = [0 as GLfloat; 3];
                    assert_eq!(
                        tok_iter
                            .take(3)
                            .enumerate()
                            .map(|(i, s)| {
                                val[i] = s.parse::<GLfloat>().unwrap();
                            })
                            .count(),
                        3
                    );
                    vn.extend(val);
                }
                Some("f") => {
                    let mut val = [[0 as GLfloat; 8]; 3];
                    assert_eq!(
                        tok_iter
                            .take(3)
                            .enumerate()
                            .map(|(i, s)| {
                                let (first, second) = s.split_once('/').unwrap();
                                let (second, third) = second.split_once('/').unwrap();

                                let first = first.parse::<usize>().unwrap() - 1;
                                let second = second.parse::<usize>().unwrap() - 1;
                                let third = third.parse::<usize>().unwrap() - 1;

                                val[i][0] = v[3 * first];
                                val[i][1] = v[3 * first + 1];
                                val[i][2] = v[3 * first + 2];
                                val[i][3] = vt[2 * second];
                                val[i][4] = vt[2 * second + 1];
                                val[i][5] = vn[2 * third];
                                val[i][6] = vn[2 * third + 1];
                                val[i][7] = vn[2 * third + 2];
                            })
                            .count(),
                        3
                    );
                    array_buf.extend(val);
                }
                _ => {}
            }
        }

        Self { buf: array_buf }
    }
}
