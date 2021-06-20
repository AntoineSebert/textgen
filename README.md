# textgen

Convincing randomly generated nonsense from a corpus, based on sequence adjacency probability

## Usage

You can either use textgen from the command line, or as a library.

### Command line

```
USAGE:
    textgen [FLAGS] [OPTIONS] <input>

ARGS:
    <input>    Input for the program

FLAGS:
    -h, --help          Prints help information
    -i, --inline        Inline, the input is used as corpus
    -s, --source        The adjacency matrix source is the output
    -t, --token-mode    In token mode, the program generates tokens until count. If this option is
                        not present, the program defaults to Sentence mode In this mode, the program
                        will try to generate sentences starting by a capitalized letter (if any),
                        and ending by a dot
    -V, --version       Prints version information

OPTIONS:
    -c, --count <count>
            Count of entities (sentences or tokens) in the output text [default: 5]

    -k, --key-length <key_length>        Sets the length of the keys in chars [default: 3]
    -v, --value-length <value_length>    Sets the length of the values in chars [default: 3
```

#### Example

`cargo r data/text_fr.txt -k 5 -v 2`
> D’ailleurs, capitaines milieu où ces abîmes de ce qui admettaient de dix-sept pas les journée de pouvait rencontraire, d’une énorme, » un ouvrait un foret peuvent habitent embarqués. Ses vingt-quatre chacun sache bientôt, collé au passage quelle cheva au milieu d’un danger réel, sérieux à éviter. Les arsenaux furent mis sur les opinion partiments des pays grandioses d’un retentissement. Or, ce cétacés, d’esprit public admit sans : d’un articulièrement observationale, et le monstre revenais d’un articulièrement à ce poindre à plus scrupuleuse attendaient pouvait-il faut nécessairement dans cette dimension, quand il affirmer, cependant de l’Union public se déclarèrent de certainement française laissait de beaux steamers. Le 13 avril 1867, la mer, de deux mille et le savoir examinèrent à l’emporte-pièce. Il fallait y renoncer. À mon arrivé à l’autres ont été envahissement. Il reconnu jusqu’au Kraken démesurassentier pour un démesuré, dont la pier de deux où l’homme ne m’appartiments pieds. Trêve à ces circonstances rivales. Donc, la flottante, d’un intelligent invités à la nation transocéanienne n’a été couronnée, on plaisants pieds anglais, puissance de navire apparitimes, furent au commodore Fitz-James à la mer, ils adjurèrent embarqués. Ses roues d’eau, profession de ces abîmes reproduisirent ressent et scientifique entre Liverpool, il existait, l’événement été perdus connues de l’objet, s’élève pas devoir renaître dans la carte ne manquait dans cette Licorne eût continuer ainsi s’expliqué et ils adjurèrent là une longueur minimum du monde savoir vu, étant à l’état-major du comme s’ingénie à la vitesse, et de ces variétés savantes, il avait dû se rendit par une demi-journée de l’Australie. Les armes de ce former une partient à son rôle d’équipage.

### Library

You can find examples in the code files, as documentation comments.

## Data

In the [data](data) folder, you can find sample corpuses in french, english, danish, feroese and romanian. The corpus files does not have to be present there for the program to work. The longer the corpus, the more accurate the output; but of course the processing time will grow as well.
