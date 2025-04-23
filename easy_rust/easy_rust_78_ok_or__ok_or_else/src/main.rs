// ok - Result to Option
// ok_or - Option to Result
// ok_or_else - Option to Result with closure
//

// Everything before main() is exactly the same
#[derive(Debug)]
struct Company {
    name: String,
    ceo: Option<String>,
}

impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None,
            ceo => Some(ceo.to_string()),
        };
        Self {
            name: name.to_string(),
            ceo,
        }
    }

    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }
}

fn main() {
    let user_input = vec![
        "8.9",
        "Nine point nine five",
        "8.0",
        "7.6",
        "eleventy-twelve",
    ];

    let actual_numbers = user_input
        .into_iter()
        .filter_map(|input| input.parse().ok()) // Ok(num) / Err(err)
        .collect::<Vec<f32>>();

    println!("{actual_numbers:?}");

    let company_vec = vec![
        Company::new("Umbrella Corporation", "Unknown"),
        Company::new("Ovintiv", "Doug Suttles"),
        Company::new("The Red-Headed League", ""),
        Company::new("Stark Enterprises", ""),
    ];
    println!("{:#?}",company_vec);

    let mut results_vec = vec![];

    company_vec 
        .iter()
        .for_each(|company| {
            //results_vec.push(company.get_ceo().ok_or("No CEO found")); 좀 더 편리한 함수는 ok_or_else
            results_vec.push(company.get_ceo().ok_or_else(||{
                let err_message = format!("No CEO found for {}", company.name);
                println!("{err_message}");
                err_message
            }))
        });

    println!("results_vec is {results_vec:?}");

    let mut results_vec1 = vec![];
    company_vec
        .iter()
        .for_each(|company| {
            results_vec1.push(company.get_ceo().ok_or_else(|| {
                let err_message = format!("No CEO found for {}", company.name);
                println!("{err_message}");
                err_message
            }))
        });
    
    println!("results_vec1 is {results_vec1:?}");

    let company_vec2 = vec![
        Company::new("Umbrella Corporation", "Unknown"),
        Company::new("Ovintiv", "Doug Suttles"),
        Company::new("The Red-Headed League", ""),
        Company::new("Stark Enterprises", ""),
    ];

    let mut results_vec2 = vec![];

    company_vec2
        .iter()
        .for_each(|company| {
            results_vec2.push(company.get_ceo().ok_or_else(||{
                let err_message = format!("No CEO 2 found for {}", company.name);
                println!("Oh no! Error: {err_message}");
                err_message
            }))
        });
    
    println!("{results_vec2:#?}");
}
