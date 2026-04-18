pub trait Bank {
    fn get_extra_data(&self, b: &BankType) -> String {
        match b {
            BankType::Monzo => println!("Monzo Bank"),
            BankType::Halifax => println!("Halifax Bank"),
        }

        String::from("")
    }
    fn get_dataframe(&self, u: &Upload) -> String {
        String::from("Not implemented")
    }
}

pub enum BankType {
    Monzo,
    Halifax,
}

pub struct Upload {
    file_contents: &'static [u8],
    bank_type: BankType,
}

impl Bank for Upload {}

pub fn switcher() -> String {
    let u: Upload = Upload {
        file_contents: "carvan".as_bytes(),
        bank_type: BankType::Monzo,
    };

    match u.bank_type {
        BankType::Monzo => u.get_dataframe(&u),
        BankType::Halifax => u.get_extra_data(&u.bank_type),
    }
}
