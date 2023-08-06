// Belirli bir koşula uygun öğeleri filtrelemek için kullanılacak bir yapı (struct) tanımlanır.
struct FilterCondition<T> {
    condition: T,
}

// `FilterCondition` yapısına, belirtilen koşula uygunluğu kontrol eden bir metod eklenir.
impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        item == &self.condition
    }
}

// Bir koleksiyonu (örneğin bir vektör) ve bir `FilterCondition` nesnesini alarak, koşula uygun olan öğeleri filtreleyen bir işlev tanımlanır.
fn custom_filter<T>(collection: &[T], filter_condition: &FilterCondition<T>) -> Vec<T>
where
    T: PartialEq + Clone,
{
    let mut filtered_collection = Vec::new();
    for item in collection {
        if filter_condition.is_match(item) {
            filtered_collection.push(item.clone());
        }
    }
    filtered_collection
}

fn main() {
    // `main` fonksiyonunda, bir vektör oluşturulur. Bu vektör içinde bir dizi sayı içerir.
    let numbers = vec![1, 2, 3, 4, 5];

    // `FilterCondition` yapısından bir nesne oluşturulur. Bu nesne, filtreleme için kullanılacak olan koşulu içerir.
    let filter_condition = FilterCondition { condition: 3 };

    // `custom_filter` işlevi çağrılır ve vektör ile `FilterCondition` nesnesi işleve geçirilir.
    // İşlev, verilen koşula uyan elemanlardan oluşan yeni bir vektörü döndürür.
    let filtered_numbers = custom_filter(&numbers, &filter_condition);

    // Sonuç ekrana yazdırılır. Filtrelenmiş sonuç, `custom_filter` işlevinden dönen yeni vektördür.
    println!("{:?}", filtered_numbers); // Çıktı: [3]
}
