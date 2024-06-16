mod application;
pub mod models;

enum AppleVariety {
    GoldenDelicious,
    GrannySmith,
    Fuji,
}

enum BananaVariety {
    Cavendish,
    GrosMichel,
    Manzano,
}

enum CherryVariety {
    Montmorency,
    Bing,
}

#[derive(Eq, PartialEq)]
struct CheckNumber(String);

struct CardNumber(String);

enum Currency {
    Eur,
    Usd,
}

enum CardType {
    Visa,
    MasterCard,
}

struct CreditCardInfo {
    card_number: CardNumber,
    card_type: CardType,
}

enum PaymentMethod {
    Cash,
    Check(CheckNumber),
    Card(CreditCardInfo),
}

struct PaymentAmount(f64);

struct Payment {
    amount: PaymentAmount,
    method: PaymentMethod,
    currency: Currency,
}
