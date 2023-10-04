/* exer*/
/* exercitiul 1 lab4
fn next_prime(x: u16) -> Option<u16>
{
    let mut i = x;
    let mut j = 3;
    let mut cautare = true;
    let mut ok:bool = true;
    let base: i32 = 2;
    if i32::from(i)>=base.pow(16) - 1 || i >= 65019
        {
            return None;
        }
        else {
    if x % 2 == 1
    {
        i = x +2;
    }
    else {
        i = x +1;
    }
        }
    while cautare{
        if i32::from(i)>=base.pow(16) || i == 0
        {
            break;
        }
        while j * j <= i && ok && i32::from(i)<=base.pow(16) - 1{
            if i%j == 0
            {
                i+=2;
                j=3;
                ok = false;
            }
            j+=2;
        }
        if ok == true
        {
            cautare = false;
        }
        ok = true;
    }
    if cautare == false
    {
        return Some(i);
    }
    else {
        return None;
    }
}

fn main()
{
    let f = next_prime(23742); //maxim 65.536
    if f.is_some(){
        println!("am gasit nr prim : {}", f.unwrap());
    }
    else {
        println!("eroare");
    }

}

//while let Some(x:u16) = next_prime(number){
    //println!(" {}",number);
   // number =x;
//}
*/
/* ---laboratorul 5
fn rot13() {
    let s: &str = "broasca";
    let mut x: u8;
    for i in s.chars() {
        if i.is_ascii() {
            if i >= 'a' && i <= 'z' {
                x = i as u8;
                x = x + 13;
                if x > 122 {
                    x = x - 27;
                }
                print!("{}", x as char);
            } else if i >= 'A' && i <= 'Z' {
                x = i as u8;
                x = x + 13;
                if x > 90 {
                    x = x - 27;
                }
                print!("{}", x as char);
            } else {
                print!("{}", i);
            }
        } else {
            print!("Error!");
            break;
        }
    }
    print!("\n");
}

fn abrevieri() {
    let s: &str = "Am fost la dl Matei pt că m-a invitat cu o zi înainte si ptr a ma imprieteni cu dna elena";
    let mut p: String = String::new();
    for i in s.split(" ") {
        if i.contains("dl") {
            p.push_str("domnul ");
        } else if i.contains("ptr") || i.contains("pt"){
            p.push_str("pentru ");
        } else if i.contains("dna"){
            p.push_str("doamna ");
        }
        else{
            p.push_str(i);
            p.push_str(" ");
        }
    }
    println!("{}", p);
}

use std::{io, fs};
//incomplet
fn do_stuff() -> Result<(), io::Error> {
    let s = fs::read_to_string("C:\\Windows\\System32\\drivers\\etc\\hosts")?;
    let mut cnt=0;
    let mut p: String = String::new();
    for i in s.split("\n") {
        if !(i.starts_with("#"))
        {
                for j in i.split(" ")
                {
                    cnt=cnt+1;
                    //
                    if cnt==2{
                        p.insert_str(0,j);
                        break;
                    }
                    else {
                        p.push_str(j);
                    }
                    //p.insert_str(0,"->");
                }
                cnt=0;
        }
    }
    fs::write("fisier.txt", &p)?;

    Ok(())
}
fn main() {
    //are len este capacitatea
    let s = "strings are fun
🎁🎶🎉👀🎈🎃🍕☕🍉
rust
supercalifragilisticexpialidocious";
    let mut max_lungime = 0;
    let mut max_capacitate = 0;
    let mut contor = 0;
    let mut poz1 = 0;
    let mut poz2 = 0;

    //calcul pt valori
    for i in s.split("\n") {
        contor = contor + 1;
        if i.chars().count() > max_lungime {
            max_lungime = i.chars().count();
            poz1 = contor;
        }
        if i.len() > max_capacitate {
            max_capacitate = i.len();
            poz2 = contor;
        }

        //println!("Stringul este {} unde are {} lungime && {} capacitate ", i, i.chars().count(), i.len());

        //println!("capacity {} && length {}", str.capacity , str.len)
    }
    contor = 0;
    for i in s.split("\n") {
        contor = contor + 1;
        if contor == poz1 {
            println!(
                "Stringul este {} unde are {} lungime && {} nr_bytes ",
                i,
                i.chars().count(),
                i.len()
            );
        }
        if contor == poz2 {
            println!(
                "Stringul este {} unde are {} lungime && {} nr_bytes ",
                i,
                i.chars().count(),
                i.len()
            );
        }

        //println!("Stringul este {} unde are {} lungime && {} capacitate ", i, i.chars().count(), i.len());

        //println!("capacity {} && length {}", str.capacity , str.len)
    }

    rot13();
    abrevieri();
    do_stuff();
}
*/

//pt struct
/*

#[derive(Debug,Clone)]
struct Persoana
{
    nume: String,
    nrtel: String,
    varsta: String
}
use std::{io, fs};
fn prob1() -> Result<(), io::Error> {
    let s = fs::read_to_string("mere.txt")?;
    print!("{}\n", s);
    let mut cnt=0;
    let mut p:Persoana =Persoana { nume: String::new(), nrtel: String::new(), varsta: String::new() };
    let mut q:Persoana =Persoana { nume: String::new(), nrtel: String::new(), varsta: String::new() };
    let mut maxi=0;
    let mut mini=0;
    let mut x:i32;

    for i in s.split("\n")
    {
        for j in i.split(",")
        {
            cnt=cnt+1;
            if cnt == 3
            {

                //let my_string = j.to_string();  // `parse()` works with `&str` and `String`!
let my_int = j.parse::<i32>().unwrap();
                println!("{}", my_int);

            }
        }
        cnt=0;
    }
    println!("{},{},{}\n", p.nume, p.nrtel, p.varsta);
    Ok(())
}
fn main()
{
    prob1();
}
*/
/*
#[derive(Debug,Copy,Clone)]
struct Point<'a>
{
x: i32,
y: i32,
name: &'a str
}
fn main() {
let p:Point;
p.x=12;
p.y=23;
p.name="hey";
let b = p;
println!("{:?}",p);
}
*/

/*use std::collections::HashMap;
use std::{io, fs};

fn prob1lab7() -> Result<(), io::Error> {
    let mut map = HashMap::new();
    let s = fs::read_to_string("lab.txt")?;
    let mut length=0;
    for i in s.split(".")
    {
        for j in i.split(" ")
        {
            if j!=""
            {
                if j.len()>length
                {
                    length=j.len();
                }
            let x_i: String = j.to_lowercase();
            if map.contains_key(&(x_i))
            {
                *map.get_mut(&(x_i)).unwrap() = *map.get_mut(&(x_i)).unwrap()+1;
            }
            else
            {
                map.entry(x_i).or_insert(1);
            }
        }
        }
    }
    let mut v1=Vec::new();
    for i in map
    {
        v1.push(i);
    }
    v1.sort_by(|a,b| b.1.cmp(&a.1));

    for i in v1
    {
        if i.0.len()<length
        {
            print!("{}",i.0);
            let mut j=length-i.0.len()+1;
            while j>0
            {
                print!(" ");
                j=j-1;
            }
        }
        else {
            print!("{} ",i.0);
        }

        println!("=> {}",i.1);
    }
    Ok(())
}
fn main()
{
    prob1lab7();

}*/
/*use std::fs::File;
use std::io::{Read, Write};
use std::env;

fn main() -> Result<(), std::io::Error> {
    let mut cnt=0;
    let mut path_in:String =String::new();
    let mut path_out:String = String::new();
    for argument in env::args() {

        if cnt == 1
        {
            if argument.contains(".png")
            {
                path_in.push_str(&argument);
            }
        }
        else if cnt == 2
        {
            if argument.contains(".png")
            {
                path_out.push_str(&argument);
            }
        }
        cnt = 0;
        if argument=="--in"
        {
            cnt=1;
        }
        if argument=="--out"
        {
            cnt=2;
        }
    }

    if path_in.is_empty()
    {
        path_in.push_str("a.txt");
    }
    if path_out.is_empty()
    {
        path_out.push_str("out.txt");
    }

    println!("{} ,\n{}",path_in,path_out);

    let mut file_in = File::open(path_in)?;
    let mut file_out = File::create(path_out)?;
    let mut buffer = [0; 4];
    loop {
        let read = file_in.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        for x in &buffer[..read]
        {
            write!(&mut file_out, "{:02x}",x)?;
        }
    }

    Ok(())
}
*/

/*use std::io::Read;
use std::fs::File;
fn f<R: Read>(reader: &mut R) -> std::io::Result<()> {
    let mut fi = File::open("1.archive")?;
    let mut buffer =[0;4];
    fi.read(&mut buffer)?;
    print!("{:?}", buffer);
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let buff;
    f(buff);
    Ok(())
}
*/

use std::env;
use std::fs;
fn main() -> Result<(), std::io::Error> {
    print!("\n");
    //oferire de roluri
    let mut rol_tip = -1; //these things are for the parameters type output delimeter and alignment
    let mut rol_output = -1;
    let mut rol_separator = -1;
    let mut rol_aliniare = -1;
    let mut cnt = 0; //no of words
    let mut fisier: String = String::new(); //the file used as input, the next as output,if it exists
    let mut iesire: String = String::new();
    for argument in env::args() {
        if cnt == 1 {
            if argument=="help"{ //explains the syntax and stuff
                print!("\nFormatul este:\n");
                print!("cargo run <nume fisier tot cu extensie> <fisier cu extensie txt|stdout> <yes|no> <left|center|right>\n\n");
                print!("<nume fisier cu extensie> trebuie sa fie de format JSON sau CSV\n");
                print!("<fisier cu extensie txt|stdout> unde se va afisa rezultatul\n");
                print!("<yes|no> daca tabelul sa aiba liniile separate de forma +____+____+\n");
                print!("<left|center|right> cum sa fie aliniate in celulele din tabel\n");
                print!("\nUltimii 3 parametri pot sa nu existe ca se ia situatia de default: stdout,no,left\n");
                return Ok(());
            }
            if argument.contains(".json") { //type
                rol_tip = 0;
            } else if argument.contains(".csv") {
                rol_tip = 1;
            }
            fisier.push_str(&argument);
        }
        if cnt == 2 {
            if argument.contains(".txt") { //output
                rol_output = 1;
                iesire.push_str(&argument);
            } else if argument == "stdout" {
                rol_output = 0;
            }
        }
        if cnt == 3 || (cnt == 2 && rol_output == -1) {
            if argument == "yes" { //delimeter
                rol_separator = 1;
            } else if argument == "no" {
                rol_separator = 0;
            }
        }
        if cnt == 4
            || (cnt == 3 && (rol_separator == -1 || rol_output == -1))
            || (cnt == 2 && (rol_output == -1 && rol_separator == -1))
        {
            if argument == "right" { //alignment
                rol_aliniare = 2;
            } else if argument == "center" {
                rol_aliniare = 1;
            } else if argument == "left" {
                rol_aliniare = 0;
            }
        }
        cnt = cnt + 1;
    }

    if rol_tip == -1 {
        print!("EROARE, TIP FISIER INVALID"); //not good type ->error
        return Ok(());
    }
    if rol_output == -1 { //take default if not chosen
        rol_output = 0;
    }
    if rol_separator == -1 { //again
        rol_separator = 0;
    }
    if rol_aliniare == -1 { //default
        rol_aliniare = 0;
    }

    //citire fisier
    let contents = fs::read_to_string(fisier).expect("Nu s-a putut citi fisierul"); //reading file

    //luare pe cazuri (daca e json sau csv)
    if rol_tip == 0 
    //JSON
    {
        let mut cuv = 0;
        let mut k = 0;
        let mut rand = 0;
        let mut again = String::with_capacity(contents.capacity());
        let mut vec = Vec::new();
        let mut nr_inregistrari = 0;
        for i in contents.lines() { //splitting up
            if i.contains(":") {
                cuv = cuv + 1;
                for j in i.split(":") {
                    let ki = &j[0..j.len() - 1];
                    again.push_str(&ki.replace(" ", ""));
                    again.push_str("^");
                    break;
                }
            }
            if i.contains("}") {
                break;
            }
        }
        let lungime = cuv;
        while cuv > 0 {
            vec.push(0);
            cuv = cuv - 1;
        }
        let mut maxi = 0;
        for i in contents.lines() {
            if i.contains(":") {
                cuv = cuv + 1;
            }
            for j in i.split(":") { //also if it takes this pattern
                if j.contains("[") {
                    if rand == 1 {
                        k = 1;
                    } else {
                        rand = 1;
                    }
                } else if j.contains("]") {
                    k = 0;
                } else if !j.contains("{") && !j.contains("}") {
                    if j.ends_with(",") {
                        let ki = &j[0..j.len() - 2];
                        again.push_str(&ki.replace(" ", ""));
                        if ki.matches(" ").count() + 1 == ki.len() {
                            again.push_str("-");
                        }
                        if k == 0 {
                            again.push_str("^");
                        } else {
                            again.push_str(" ");
                        }
                        if maxi != 0 && k == 0 {
                            if maxi > vec[cuv - 1] {
                                vec[cuv - 2] = maxi;
                            }
                            maxi = 0;
                        }
                        maxi = maxi + ki.replace(" ", "").len() - 1;
                        if k == 0 {
                            if maxi > vec[cuv - 1] {
                                vec[cuv - 1] = maxi;
                            }
                            maxi = 0;
                        }
                    } else {
                        let ki = &j[0..j.len() - 1];
                        again.push_str(&ki.replace(" ", ""));
                        again.push_str("^");
                        if maxi != 0 && k == 0 {
                            if maxi > vec[cuv - 1] {
                                vec[cuv - 2] = maxi;
                            }
                            maxi = 0;
                        }
                        if maxi != 0 {
                            maxi = maxi + 1;
                        }
                        maxi = maxi + ki.replace(" ", "").len() - 1;
                        if k == 0 {
                            if maxi > vec[cuv - 1] {
                                vec[cuv - 1] = maxi;
                            }
                            maxi = 0;
                        }
                    }
                }
            }
            if cuv == lungime { //how many groups
                cuv = 0;
                nr_inregistrari = nr_inregistrari + 1;
            }
        }
        //acum incepe scrierea tabelului

        let mut finalul = String::with_capacity(contents.capacity() * contents.capacity());
        let mut pat = String::with_capacity(contents.capacity()); //creating the final array ; the solution
        finalul.push_str("+");
        pat.push_str("+");
        cuv = 0;
        maxi = 0;
        while cuv < lungime {
            while maxi < vec[cuv] {
                finalul.push_str("-");
                pat.push_str("_");
                maxi = maxi + 1;
            }
            finalul.push_str("+");
            pat.push_str("+");
            maxi = 0;
            cuv = cuv + 1;
        }
        finalul.push_str("\n");
        pat.push_str("\n");

        //continut tabel
        cuv = 0;
        maxi = 0;
        let mut contor = 0;
        let mut param = 0;
        let mut ajutor = 0;
        finalul.push_str("|");
        if rol_aliniare == 0 || rol_aliniare == 2 { //with cases of aligment
            for i in again.replace("\"", "").split("^") {
                contor = contor + 1;
                if contor <= lungime {
                    param = 1;
                } else {
                    param = 2;
                }
                if param == 1 {
                    cuv = cuv + 1;
                    if rol_aliniare == 0 {
                        finalul.push_str(i);
                    }
                    while maxi < vec[cuv - 1] - i.len() {
                        finalul.push_str(" ");
                        maxi = maxi + 1;
                    }
                    if rol_aliniare == 2 {
                        finalul.push_str(i);
                    }
                    finalul.push_str("|");
                    maxi = 0;
                    if cuv == lungime {
                        cuv = 0;
                        finalul.push_str("\n");
                        if contor == lungime {
                            finalul.push_str(&pat);
                        }
                        finalul.push_str("|");
                    }
                } else {
                    ajutor = ajutor + 1;
                    maxi = 0;
                    if ajutor % 2 == 0 {
                        cuv = cuv + 1;
                        if rol_aliniare == 0 {
                            finalul.push_str(i);
                        }
                        while maxi < vec[cuv - 1] - i.len() {
                            finalul.push_str(" ");
                            maxi = maxi + 1;
                        }
                        if rol_aliniare == 2 {
                            finalul.push_str(i);
                        }
                        finalul.push_str("|");
                        maxi = 0;
                        if cuv == lungime {
                            cuv = 0;
                            finalul.push_str("\n");
                            if contor == lungime {
                                finalul.push_str(&pat);
                            }
                            if contor == nr_inregistrari * lungime * 2 + lungime {
                                break;
                            }
                            if rol_separator == 1 && contor != lungime {
                                finalul.push_str(&pat);
                            }
                            finalul.push_str("|");
                        }
                    }
                }
            }
            finalul.push_str(&pat);
        } else if rol_aliniare == 1 {
            for i in again.replace("\"", "").split("^") {
                contor = contor + 1;
                if contor <= lungime {
                    param = 1;
                } else {
                    param = 2;
                }
                if param == 1 {
                    cuv = cuv + 1;
                    while maxi < (vec[cuv - 1] - i.len()) / 2 {
                        finalul.push_str(" ");
                        maxi = maxi + 1;
                    }
                    finalul.push_str(i);
                    maxi = 0;
                    while maxi < (vec[cuv - 1] - i.len()) / 2 + (vec[cuv - 1] - i.len()) % 2 {
                        finalul.push_str(" ");
                        maxi = maxi + 1;
                    }
                    finalul.push_str("|");
                    maxi = 0;
                    if cuv == lungime {
                        cuv = 0;
                        finalul.push_str("\n");
                        if contor == lungime {
                            finalul.push_str(&pat);
                        }
                        finalul.push_str("|");
                    }
                } else {
                    ajutor = ajutor + 1;
                    maxi = 0;
                    if ajutor % 2 == 0 {
                        cuv = cuv + 1;
                        while maxi < (vec[cuv - 1] - i.len()) / 2 {
                            finalul.push_str(" ");
                            maxi = maxi + 1;
                        }
                        finalul.push_str(i);
                        maxi = 0;
                        while maxi < (vec[cuv - 1] - i.len()) / 2 + (vec[cuv - 1] - i.len()) % 2 {
                            finalul.push_str(" ");
                            maxi = maxi + 1;
                        }
                        finalul.push_str("|");
                        maxi = 0;
                        if cuv == lungime {
                            cuv = 0;
                            finalul.push_str("\n");
                            if contor == lungime {
                                finalul.push_str(&pat);
                            }
                            if contor == nr_inregistrari * lungime * 2 + lungime {
                                break;
                            }
                            if rol_separator == 1 && contor != lungime {
                                finalul.push_str(&pat);
                            }
                            finalul.push_str("|");
                        }
                    }
                }
            }
            finalul.push_str(&pat);
        }
        if rol_output == 0 { //stdout just showing it
            print!("{finalul}");
        } else {
            //deschidere fisier
            fs::write(iesire, finalul).expect("Nu s-a putut scrie");
        }
        print!("\n");
    } else {
        //CSV
        let mut cuv = 0;
        let mut k = 0;
        let mut again = String::with_capacity(contents.capacity()); //the same steps
        let mut vec = Vec::new();
        let mut nr_inregistrari = 0;
        for i in contents.lines() {
            for j in i.split(",") {
                if j.contains("\"") {
                    k = k + j.matches("\"").count();
                }
                if k % 2 == 0 {
                    cuv = cuv + 1;
                    k = 0;
                }
            }
            if k % 2 == 0 {
                break;
            }
        }
        let lungime = cuv;
        while cuv > 0 {
            vec.push(0);
            cuv = cuv - 1;
        }
        let mut maxi = 0;
        for i in contents.lines() {
            for j in i.split(",") {
                if j.starts_with("\"") {
                    if j == "\"\"" {
                        again.push_str("-");
                    } else {
                        let mut ki = j.clone();
                        if (ki.ends_with("\"") && !ki.ends_with("\"\"")) || ki.ends_with("\"\"\"") {
                            ki = &ki[1..ki.len() - 1];
                        } else {
                            ki = &ki[1..];
                        }
                        again.push_str(&ki.replace("\"\"", "\""));
                    }
                } else {
                    let mut ki = j.clone();
                    if (ki.ends_with("\"") && !ki.ends_with("\"\"")) || ki.ends_with("\"\"\"") {
                        ki = &ki[0..ki.len() - 1];
                    } else {
                        ki = &ki[0..];
                    }
                    again.push_str(&ki.replace("\"\"", "\""));
                }
                maxi = maxi + j.len();
                if j.contains("\"") {
                    k = k + j.matches("\"").count();
                }
                if k % 2 == 0 {
                    again.push_str("^");
                    if k != 0 {
                        maxi = maxi - 2 - (k - 2) % 2;
                    }
                    if maxi > vec[cuv] {
                        vec[cuv] = maxi;
                    }
                    cuv = cuv + 1;
                    maxi = 0;
                    k = 0;
                }
            }
            if k % 2 == 0 {
                cuv = 0;
                nr_inregistrari = nr_inregistrari + 1;
            }
        }

        //acum incepe scrierea tabelului

        let mut finalul = String::with_capacity(contents.capacity() * contents.capacity());
        let mut pat = String::with_capacity(contents.capacity());
        finalul.push_str("+");
        pat.push_str("+");
        cuv = 0;
        maxi = 0;
        while cuv < lungime {
            while maxi < vec[cuv] {
                finalul.push_str("-");
                pat.push_str("_");
                maxi = maxi + 1;
            }
            finalul.push_str("+");
            pat.push_str("+");
            maxi = 0;
            cuv = cuv + 1;
        }
        finalul.push_str("\n");
        pat.push_str("\n");

        //continut tabel
        cuv = 0;
        maxi = 0;
        let mut contor = 0;
        finalul.push_str("|");
        if rol_aliniare == 0 || rol_aliniare == 2 {
            for i in again.split("^") {
                contor = contor + 1;
                cuv = cuv + 1;
                if rol_aliniare == 0 {
                    finalul.push_str(i);
                }
                while maxi < vec[cuv - 1] - i.len() {
                    finalul.push_str(" ");
                    maxi = maxi + 1;
                }
                if rol_aliniare == 2 {
                    finalul.push_str(i);
                }
                finalul.push_str("|");
                maxi = 0;
                if cuv == lungime {
                    cuv = 0;
                    finalul.push_str("\n");
                    if contor == lungime {
                        finalul.push_str(&pat);
                    }
                    if contor == nr_inregistrari * lungime {
                        break;
                    }
                    if rol_separator == 1 && contor != lungime {
                        finalul.push_str(&pat);
                    }
                    finalul.push_str("|");
                }
            }
            finalul.push_str(&pat);
        } else if rol_aliniare == 1 {
            for i in again.split("^") {
                contor = contor + 1;
                cuv = cuv + 1;
                while maxi < (vec[cuv - 1] - i.len()) / 2 {
                    finalul.push_str(" ");
                    maxi = maxi + 1;
                }
                finalul.push_str(i);
                maxi = 0;
                while maxi < (vec[cuv - 1] - i.len()) / 2 + (vec[cuv - 1] - i.len()) % 2 {
                    finalul.push_str(" ");
                    maxi = maxi + 1;
                }
                finalul.push_str("|");
                maxi = 0;
                if cuv == lungime {
                    cuv = 0;
                    finalul.push_str("\n");
                    if contor == lungime {
                        finalul.push_str(&pat);
                    }
                    if contor == nr_inregistrari * lungime {
                        break;
                    }
                    if rol_separator == 1 && contor != lungime {
                        finalul.push_str(&pat);
                    }
                    finalul.push_str("|");
                }
            }
            finalul.push_str(&pat);
        }
        if rol_output == 0 {
            print!("{finalul}");
        } else {
            //deschidere fisier
            fs::write(iesire, finalul).expect("Nu s-a putut scrie");
        }
        print!("\n"); //and thats all =) if u want i could show you
        //took default cases
        //but if you dont respect the order, the rest are ignored or smth look
        //its strict
    }
    Ok(())
}
