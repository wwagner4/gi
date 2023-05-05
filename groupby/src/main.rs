use itertools::Itertools;

#[derive(Debug, Clone)]
struct MasterData {
    id: i32,
    master_name: String,
}

#[derive(Debug, Clone)]
struct CombiData {
    master: MasterData,
    datas: Vec<Data>,
}

#[derive(Debug, Clone)]
struct CombiDataRef<'a> {
    master: &'a MasterData,
    datas: Vec<&'a Data>,
}

#[derive(Debug, Clone)]
struct Data {
    id: i32,
    username: String,
}

fn main() {
    let master_datas = vec![
        MasterData { id: 1, master_name: "X".to_string() },
        MasterData { id: 2, master_name: "Y".to_string() },
        MasterData { id: 3, master_name: "Z".to_string() },
    ];

    let datas = vec![
        Data { id: 1, username: "A".to_string() },
        Data { id: 1, username: "B".to_string() },
        Data { id: 3, username: "C".to_string() },
        Data { id: 2, username: "D".to_string() },
        Data { id: 2, username: "E".to_string() },
    ];

    println!("- filter ref -----------------------------------");
    let data_groups = master_datas
        .iter()
        .map(|md| {
            let datas = datas
                .iter()
                .filter(|d| d.id == md.id)
                .collect::<Vec<_>>();
            CombiDataRef { master: &md, datas }
        })
        .collect::<Vec<_>>();
    for grp in data_groups {
        println!("grp: {:?}", grp)
    }

    println!("- filter owned -----------------------------------");
    let data_groups = master_datas
        .into_iter()
        .map(|master| {
            let datas = datas
                .iter()
                .filter(|d| d.id == master.id)
                .map(|d| d.to_owned())
                .collect::<Vec<_>>();
            CombiData { master, datas }
        })
        .collect::<Vec<_>>();
    for grp in data_groups {
        println!("grp: {:?}", grp)
    }

    println!("- group_by -----------------------------------");
    let groups: Vec<_> = datas
        .iter()
        .group_by(|x| x.id)
        .into_iter()
        .map(|(a, b)| (a, b.collect::<Vec<_>>()))
        .collect::<Vec<_>>();
    for grp in groups {
        println!("grp: {:?}", grp);
    }
}
