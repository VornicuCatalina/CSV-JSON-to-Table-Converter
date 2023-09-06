use std::env;
use std::fs;
fn main() -> Result<(), std::io::Error> {
    print!("\n");
    //oferire de roluri
    let mut rol_tip = -1;
    let mut rol_output = -1;
    let mut rol_separator = -1;
    let mut rol_aliniare = -1;
    let mut cnt = 0;
    let mut fisier: String = String::new();
    let mut iesire: String = String::new();
    for argument in env::args() {
        if cnt == 1 {
            if argument=="help"{
                print!("\nFormatul este:\n");
                print!("cargo run <nume fisier tot cu extensie> <fisier cu extensie txt|stdout> <yes|no> <left|center|right>\n\n");
                print!("<nume fisier cu extensie> trebuie sa fie de format JSON sau CSV\n");
                print!("<fisier cu extensie txt|stdout> unde se va afisa rezultatul\n");
                print!("<yes|no> daca tabelul sa aiba liniile separate de forma +____+____+\n");
                print!("<left|center|right> cum sa fie aliniate in celulele din tabel\n");
                print!("\nUltimii 3 parametri pot sa nu existe ca se ia situatia de default: stdout,no,left\n");
                return Ok(());
            }
            if argument.contains(".json") {
                rol_tip = 0;
            } else if argument.contains(".csv") {
                rol_tip = 1;
            }
            fisier.push_str(&argument);
        }
        if cnt == 2 {
            if argument.contains(".txt") {
                rol_output = 1;
                iesire.push_str(&argument);
            } else if argument == "stdout" {
                rol_output = 0;
            }
        }
        if cnt == 3 || (cnt == 2 && rol_output == -1) {
            if argument == "yes" {
                rol_separator = 1;
            } else if argument == "no" {
                rol_separator = 0;
            }
        }
        if cnt == 4
            || (cnt == 3 && (rol_separator == -1 || rol_output == -1))
            || (cnt == 2 && (rol_output == -1 && rol_separator == -1))
        {
            if argument == "right" {
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
        print!("EROARE, TIP FISIER INVALID");
        return Ok(());
    }
    if rol_output == -1 {
        rol_output = 0;
    }
    if rol_separator == -1 {
        rol_separator = 0;
    }
    if rol_aliniare == -1 {
        rol_aliniare = 0;
    }

    //citire fisier
    let contents = fs::read_to_string(fisier).expect("Nu s-a putut citi fisierul");

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
        for i in contents.lines() {
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
            for j in i.split(":") {
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
            if cuv == lungime {
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
        let mut param = 0;
        let mut ajutor = 0;
        finalul.push_str("|");
        if rol_aliniare == 0 || rol_aliniare == 2 {
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
        if rol_output == 0 {
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
        let mut again = String::with_capacity(contents.capacity());
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
        print!("\n");
    }
    Ok(())
}
