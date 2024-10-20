fn main() {

    calculator(12, 13, 5);

}

fn calculator(sayi:i32,sayi2:i32,değer:i32) {


    if değer==1 {

        let topla = sayi+sayi2;

    println!("toplamanın sonucu {}", topla);
        
    }
    else if değer==2 {

        let çarpma = sayi*sayi2;

    println!("çarpmanın sonucu {}", çarpma);
        
    }else if değer==3 {

        let çıkarma = sayi-sayi2;

    println!("çıkarmanın sonucu {}", çıkarma);
        
    }else {
        println!("3 den büyük değer girdiniz {} " , değer)
    }

    


    
    
}
