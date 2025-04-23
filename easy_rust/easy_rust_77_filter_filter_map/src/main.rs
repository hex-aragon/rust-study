//filter = 조건있는 것만 전달?
//filter_map = 필터 + 맵 


struct Company {
    name: String, 
    ceo : Option<String>
}

impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None, 
            ceo => Some(ceo.to_string())
        };

        Self {
            name: name.to_string(),
            ceo
        }
    }

    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }
}

fn main() {
    let months = vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let filtered_months = months 
        .into_iter()
        .filter(|months| months.len() < 5 ) //true or false로 bool 타입이 와야 한다. 
        //.filter(|months| months) //expected `bool`, found `&&str` 
        .filter
        (|monnnth| monnnth.contains("u"))
        .collect::<Vec<&str>>();

    println!("{filtered_months:?}");


    let company_vec = vec![
        Company::new("Umbrella Corporation", "Unknown"),
        Company::new("Ovintiv", "Doug Suttles"),
        Company::new("The Red-Headed League", ""),
        Company::new("Stark Enterprises", ""),
    ];

    let all_the_ceos = company_vec
            .into_iter()
            .filter_map(|company| company.get_ceo()) // Some / None 
            .collect::<Vec<_>>();
    println!("{all_the_ceos:?}");

    //필터와 맵을 따로 쓰는 것 보다는 필터맵을 쓰는게 좋다.
    let a = ["1", "two", "NaN", "four", "5"];
    let mut iter = a.iter().filter_map(|s| s.parse().ok());

    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(5));
    assert_eq!(iter.next(), None);
    
}
