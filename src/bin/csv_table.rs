use std::convert::TryInto;
fn main() {
    let input = "Name,Street,City,Age
Peter Pan,Am Hang 5,12345 Einsam,42
Maria Schmitz,Kölner Straße 45,50123 Köln,43
Paul Meier,Münchener Weg 1,87654 München,65";
    let mut rows: Vec<&str> = input.split("\n").collect();
    let headers: Vec<&str> = rows[0].split(",").collect();
    rows.remove(0);
    let mut values: Vec<Vec<&str>> = vec![];
    for _ in 0..headers.len() {
        values.push(Vec::new())
    }
    for v in rows.clone() {
        let vals: Vec<&str> = v.split(",").collect();
        for (a, b) in vals.iter().enumerate() {
            values[a].push(b)
        }
    }
    let mut max_len: Vec<usize> = vec![];
    for (index, i) in values.iter().enumerate() {
        let mut len = 0;
        for x in i {
            len = if x.len() > len { x.len() } else { len };
        }
        if headers[index].len() > len {
            len = headers[index].len()
        }
        max_len.push(len.try_into().unwrap())
    }

    println!("{}", {
        let mut ree: Vec<String> = vec![];
        for (i, e) in headers.iter().enumerate() {
            let mut a = String::new();
            let c = e.to_string();
            let b: &str = &format!("{:1$}", c, max_len[i]).to_string();
            a.push_str(b);
            ree.push(a)
        }
        ree.join(" | ")
    });
    println!("{}", {
        let mut c: Vec<String> = vec![];
        for (i, _) in headers.iter().enumerate() {
            c.push(String::from_utf8(vec![b'-'; max_len[i]]).expect("Column truncation failed."))
        }
        c.join("-+-")
    });
    for e in rows.clone() {
        println!("{}", {
            let mut w: Vec<String> = vec![];
            for (i, x) in e.split(",").enumerate() {
                let b = format!("{:1$}", x, max_len[i]);
                let mut a = String::new();
                a.push_str(&b.to_string());
                w.push(a);
            }
            w.join(" | ")
        })
    }
}
