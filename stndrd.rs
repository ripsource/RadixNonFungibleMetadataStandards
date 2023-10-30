// Some options for consideration for formatting 'attribute/trait' metadata on Radix NFTs.

// OPTION 1: Using a map/kv of Key: String, Value: String. 

#[derive(NonFungibleData, ScryptoSbor)]
struct NFT {
    name: String,
    description: String,
    key_image_url: Url,
    attributes: HashMap<String, String>,
}

// OPTION 2: Using a Vector of Custom Structs (either A or B)

#[derive(NonFungibleData, ScryptoSbor)]
struct NFT {
    name: String,
    description: String,
    key_image_url: Url,
    attributes: Vec<NonFungibleMetaData>, // Or Vec<NonFungibleDisplayData>
}

/// A: Include just trait_type (e.g. Eyes) and a value (e.g. Lazer Beams)

#[derive(ScryptoSbor)]
struct NonFungibleData {
    trait_type: String,
    value: String
}

/// B: Optional to include a display_type field (see OpenSea standards) 

#[derive(ScryptoSbor)]
struct NonFungibleDisplayMetaData {
    display_type: String,
    trait_type: String,
    value: String
}