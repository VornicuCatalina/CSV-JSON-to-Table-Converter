Cateva lucruri specificate:
=============================================================================================================
	*Comanda este de forma: cargo run <fisier> <param1> <param2> <param3>
		-ultimii 3 parametri pot sa nu apara
	*Sau comanda de ajutoru: cargo run help
-------------------------------------------------------------------------------------------------------------
	*Modul de apelare este: ./x.exe <in.json|in.csv> <out.txt|stdout> <yes|no> <left|center|right>
-------------------------------------------------------------------------------------------------------------
	*"ceva" este un exemplu pentru JSON, iar "ceva2" este un exemplu pentru CSV.
	-la JSON trebuie sa respecte formatul urmator:
*************************************************************************************************************
[
	{
		"camp":"valoare",
		"camp":"valoare"
	},
	{
		"camp":"valoare",
		"camp":"valoare"
	}
]

*************************************************************************************************************
, unde "valoare" poate sa fie scris si asa:
*************************************************************************************************************
	"camp": [
			"valoare1",
			"valoare2"
		]
*************************************************************************************************************
	-la CSV trateaza orice caz, cat timp respecta formatul de a fi scris corespunzator:
*intre " " daca contine spatiu liber
*intre " " daca e pe mai multe randuri scris
	-daca dupa cuvant nu e spatiu liver, atunci nici programul nu il adauga, de aceea trebuie adaugat
*grupurile "" "" vor deveni " " 
-------------------------------------------------------------------------------------------------------------
	*De asemenea, valorile vide "" vor deveni "-" in tabel (pentru a nu fi spatii goale-tine de estetica).
=============================================================================================================
