use itertools::Itertools;

#[derive(Debug, Clone)]
struct Data {
    id: i32,
    username: String,
}

fn main() {
    let data = vec![
        Data { id: 1, username: "A".to_string() },
        Data { id: 1, username: "B".to_string() },
        Data { id: 3, username: "C".to_string() },
        Data { id: 2, username: "D".to_string() },
        Data { id: 2, username: "E".to_string() },
    ];

    println!("- group_by -----------------------------------");
    let grps: Vec<_> = data
        .iter()
        .group_by(|x| x.id)
        .into_iter()
        .map(|(a, b)| (a, b.collect::<Vec<_>>()))
        .collect::<Vec<_>>();
    for grp in grps {
        println!("grp: {:?}", grp);
    }

    println!("- filter -----------------------------------");
    let ids = vec![1, 2, 3, 4];
    let grps = ids.into_iter()
        .map(|id| {
            let grp = data
                .iter()
                .filter(|d| d.id == id)
                .collect::<Vec<_>>();
            (id, grp)
        })
        .collect::<Vec<_>>();
    for grp in grps {
        println!("grp: {:?}", grp)
    }
}
