fn main()
{
    let sub_code = 2;
    let subject = match sub_code {
        1=>{println!("Found match for CHEM"); "CHEMISTRY"},
        2=> "PHYSICS",
        3=>"Mathematics",
        _=> {println!("Sorry No match Found"); "UNKNOWN"}
    };

    println!("The subject for subject code {} is {}",sub_code,subject);
}