mod java {
  fn main() {
    let colors = [
      ("aliceblue"), ("f0f8ff"),
      ("aliceblue"), 0x0f0f0f,
    ("alizarin"), ("e74c3c"),
    ("amethyst"), ("9b59b6"),
    ("antiquewhite"), ("faebd7"),
    ("aqua"), ("00ffff"),
    ("aquamarine"), ("7fffd4"),
    ("asbestos"), ("7f8c8d"),
    ("azure"), ("f0ffff"),
    ("beige"), ("f5f5dc"),
    ("belizehole"), ("2980b9"),
    ("bisque"), ("ffe4c4"),
    ("black"), ("000000"),
    ("blanchedalmond"), ("ffebcd"),
    ("blue"), ("0000ff"),
    ("blueviolet"), ("8a2be2"),
    ("brown"), ("a52a2a"),
    ("buff"), ("F0DC82"),
    ("burlywood"), ("deb887"),
    ("cadetblue"), ("5f9ea0"),
    ("carrot"), ("e67e22"),
    ("chartreuse"), ("7fff00"),
    ("chocolate"), ("d2691e"),
    ("clouds"), ("ecf0f1"),
    ("concrete"), ("95a5a6"),
    ("coral"), ("ff7f50"),
    ("cornflowerblue"), ("6495ed"),
    ("cornsilk"), ("fff8dc"),
    ("crimson"), ("dc143c"),
    ("cyan"), ("00ffff"),
    ("darkblue"), ("00008b"),
    ("darkbrown"), ("5C4033"),
    ("darkbuff"), ("976638"),
    ("darkcyan"), ("008b8b"),
    ("darkgold"), ("EEBC1D"),
    ("darkgoldenrod"), ("b8860b"),
    ("darkgray"), ("404040"),
    ("darkgreen"), ("006400"),
    ("darkgrey"), ("a9a9a9"),
    ("darkivory"), ("F2E58F"),
    ("darkkhaki"), ("bdb76b"),
    ("darkmagenta"), ("8b008b"),
    ("darkmustard"), ("7C7C40"),
    ("darkolivegreen"), ("556b2f"),
    ("darkorange"), ("ff8c00"),
    ("darkorchid"), ("9932cc"),
    ("darkpink"), ("E75480"),
    ("darkred"), ("8b0000"),
    ("darksalmon"), ("e9967a"),
    ("darksilver"), ("AFAFAF"),
    ("darkseagreen"), ("8fbc8f"),
    ("darkslateblue"), ("483d8b"),
    ("darkslategray"), ("2f4f4f"),
    ("darkslategrey"), ("2f4f4f"),
    ("darkturquoise"), ("00ced1"),
    ("darkviolet"), ("9400d3"),
    ("darkyellow"), ("FFCC00"),
    ("deeppink"), ("ff1493"),
    ("deepskyblue"), ("00bfff"),
    ("dimgray"), ("696969"),
    ("dimgrey"), ("696969"),
    ("dodgerblue"), ("1e90ff"),
    ("emerald"), ("2ecc71"),
    ("firebrick"), ("b22222"),
    ("floralwhite"), ("fffaf0"),
    ("forestgreen"), ("228b22"),
    ("fuchsia"), ("ff00ff"),
    ("gainsboro"), ("dcdcdc"),
    ("ghostwhite"), ("f8f8ff"),
    ("gold"), ("ffd700"),
    ("goldenrod"), ("daa520"),
    ("gray"), ("808080"),
    ("green"), ("008000"),
    ("greensea"), ("16a085"),
    ("greenyellow"), ("adff2f"),
    ("grey"), ("808080"),
    ("honeydew"), ("f0fff0"),
    ("hotpink"), ("ff69b4"),
    ("indianred"), ("cd5c5c"),
    ("indigo"), ("4b0082"),
    ("ivory"), ("fffff0"),
    ("khaki"), ("f0e68c"),
    ("lavender"), ("e6e6fa"),
    ("lavenderblush"), ("fff0f5"),
    ("lawngreen"), ("7cfc00"),
    ("lemonchiffon"), ("fffacd"),
    ("lightblack"), ("808080"),
    ("lightblue"), ("add8e6"),
    ("lightbrown"), ("9966FF"),
    ("lightbuff"), ("ECD9B0"),
    ("lightcoral"), ("f08080"),
    ("lightcyan"), ("e0ffff"),
    ("lightgold"), ("F1E5AC"),
    ("lightgoldenrod"), ("FFEC8B"),
    ("lightgoldenrodyellow"), ("fafad2"),
    ("lightgray"), ("d3d3d3"),
    ("lightgreen"), ("90ee90"),
    ("lightgrey"), ("d3d3d3"),
    ("lightivory"), ("FFF8C9"),
    ("lightmagenta"), ("FF77FF"),
    ("lightmustard"), ("EEDD62"),
    ("lightorange"), ("D9A465"),
    ("lightpink"), ("ffb6c1"),
    ("lightred"), ("FF3333"),
    ("lightsalmon"), ("ffa07a"),
    ("lightseagreen"), ("20b2aa"),
    ("lightsilver"), ("E1E1E1"),
    ("lightskyblue"), ("87cefa"),
    ("lightslategray"), ("778899"),
    ("lightslategrey"), ("778899"),
    ("lightsteelblue"), ("b0c4de"),
    ("lightturquoise"), ("AFE4DE"),
    ("lightviolet"), ("7A5299"),
    ("lightyellow"), ("ffffe0"),
    ("lime"), ("00ff00"),
    ("limegreen"), ("32cd32"),
    ("linen"), ("faf0e6"),
    ("magenta"), ("ff00ff"),
    ("maroon"), ("800000"),
    ("mediumaquamarine"), ("66cdaa"),
    ("mediumblue"), ("0000cd"),
    ("mediumorchid"), ("ba55d3"),
    ("mediumpurple"), ("9370db"),
    ("mediumseagreen"), ("3cb371"),
    ("mediumslateblue"), ("7b68ee"),
    ("mediumspringgreen"), ("00fa9a"),
    ("mediumturquoise"), ("48d1cc"),
    ("mediumvioletred"), ("c71585"),
    ("midnightblue"), ("191970"),
    ("midnightblack"), ("2c3e50"),
    ("mintcream"), ("f5fffa"),
    ("mistyrose"), ("ffe4e1"),
    ("moccasin"), ("ffe4b5"),
    ("mustard"), ("FFDB58"),
    ("nephritis"), ("27ae60"),
    ("navajowhite"), ("ffdead"),
    ("navy"), ("000080"),
    ("oldlace"), ("fdf5e6"),
    ("olive"), ("808000"),
    ("olivedrab"), ("6b8e23"),
    ("orange"), ("ffa500"),
    ("orangered"), ("ff4500"),
    ("orchid"), ("da70d6"),
    ("palegoldenrod"), ("eee8aa"),
    ("palegreen"), ("98fb98"),
    ("paleturquoise"), ("afeeee"),
    ("palevioletred"), ("db7093"),
    ("papayawhip"), ("ffefd5"),
    ("peachpuff"), ("ffdab9"),
    ("peterriver"), ("3498db"),
    ("peru"), ("cd853f"),
    ("pink"), ("ffc0cb"),
    ("plum"), ("dda0dd"),
    ("pomegranate"), ("c0392b"),
    ("powderblue"), ("b0e0e6"),
    ("purple"), ("800080"),
    ("pumpkin"), ("d35400"),
    ("rebeccapurple"), ("663399"),
    ("red"), ("ff0000"),
    ("rosybrown"), ("bc8f8f"),
    ("royalblue"), ("4169e1"),
    ("saddlebrown"), ("8b4513"),
    ("salmon"), ("fa8072"),
    ("sandybrown"), ("f4a460"),
    ("seagreen"), ("2e8b57"),
    ("seashell"), ("fff5ee"),
    ("sienna"), ("a0522d"),
    ("quiksilver"), ("bdc3c7"),
    ("silver"), ("c0c0c0"),
    ("skyblue"), ("87ceeb"),
    ("slateblue"), ("6a5acd"),
    ("slategray"), ("708090"),
    ("slategrey"), ("708090"),
    ("snow"), ("fffafa"),
    ("springgreen"), ("00ff7f"),
    ("steelblue"), ("4682b4"),
    ("sunflower"), ("f1c40f"),
    ("tan"), ("d2b48c"),
    ("teal"), ("008080"),
    ("thistle"), ("d8bfd8"),
    ("tomato"), ("ff6347"),
    ("turquoise"), ("40e0d0"),
    ("violet"), ("ee82ee"),
    ("wetasphalt"), ("34495e"),
    ("wisteria"), ("8e44ad"),
    ("wheat"), ("f5deb3"),
    ("white"), ("ffffff"),
    ("whitesmoke"), ("f5f5f5"),
    ("yellow"), ("ffff00"),
    ("yellowgreen"), ("9acd32"),

    ("beekeeper"), ("f6e58d"),
    ("spiced"), ("ffbe76"),
    ("pink_"), ("ff7979"),
    ("june"), ("badc58"),
    ("coastal"), ("dff9fb"),
    ("turbo"), ("f9ca24"),
    ("quince"), ("f0932b"),
    ("carmine"), ("eb4d4b"),
    ("pure"), ("6ab04c"),
    ("hint_"), ("c7ecee"),
    ("middle"), ("7ed6df"),
    ("heliotrope"), ("e056fd"),
    ("exodus"), ("686de0"),
    ("deep_"), ("30336b"),
    ("soaring"), ("95afc0"),
    ("greenland"), ("22a6b3"),
    ("steel"), ("be2edd"),
    ("blurple"), ("4834d4"),
    ("deep"), ("130f40"),
    ("wizard"), ("535c68"),
    ("highlighter_"), ("ef5777"),
    ("dark_"), ("575fcf"),
    ("megaman_"), ("4bcffa"),
    ("fresh"), ("34e7e4"),
    ("minty"), ("0be881"),
    ("sizzling"), ("f53b57"),
    ("free"), ("3c40c6"),
    ("spiro_"), ("0fbcf9"),
    ("jade_"), ("00d8d6"),
    ("green_"), ("05c46b"),
    ("narenji"), ("ffc048"),
    ("yriel"), ("ffdd59"),
    ("sunset"), ("ff5e57"),
    ("hint__"), ("d2dae2"),
    ("good_"), ("485460"),
    ("chrome"), ("ffa801"),
    ("vibrant"), ("ffd32a"),
    ("red_"), ("ff3f34"),
    ("london"), ("808e9b"),
    ("black_"), ("1e272e"),
    ("flat"), ("fad390"),
    ("melon"), ("f8c291"),
    ("livid"), ("6a89cc"),
    ("spray"), ("82ccdd"),
    ("paradise"), ("b8e994"),
    ("squash"), ("f6b93b"),
    ("mandarin_"), ("e55039"),
    ("azraq"), ("4a69bd"),
    ("dupain"), ("60a3bc"),
    ("aurora"), ("78e08f"),
    ("iceland"), ("fa983a"),
    ("tomato_"), ("eb2f06"),
    ("good"), ("3c6382"),
    ("waterfall"), ("38ada9"),
    ("carrot_"), ("e58e26"),
    ("jalapeno"), ("b71540"),
    ("dark__"), ("0c2461"),
    ("forest"), ("0a3d62"),
    ("reef"), ("079992"),
    ("jigglypuff"), ("ff9ff3"),
    ("casandora"), ("feca57"),
    ("pastel"), ("ff6b6b"),
    ("megaman"), ("48dbfb"),
    ("wild"), ("1dd1a1"),
    ("lotus"), ("f368e0"),
    ("double"), ("ff9f43"),
    ("amour"), ("ee5253"),
    ("cyanite"), ("0abde3"),
    ("dark"), ("10ac84"),
    ("jade"), ("00d2d3"),
    ("joust"), ("54a0ff"),
    ("nasu"), ("5f27cd"),
    ("light_"), ("c8d6e5"),
    ("fuel"), ("576574"),
    ("aqua_"), ("01a3a4"),
    ("bleu"), ("2e86de"),
    ("bluebell_"), ("341f97"),
    ("storm"), ("8395a7"),
    ("imperial"), ("222f3e"),
    ("orchid_"), ("fea47f"),
    ("spiro_"), ("25ccf7"),
    ("honey"), ("eab543"),
    ("sweet"), ("55e6c1"),
    ("falling"), ("cad3c8"),
    ("rich"), ("f97f51"),
    ("clear"), ("1b9cfc"),
    ("sarawak"), ("f8efba"),
    ("keppel"), ("58b19f"),
    ("ships"), ("2c3a47"),
    ("fiery"), ("b33771"),
    ("bluebell"), ("3b3b98"),
    ("georgia"), ("fd7272"),
    ("oasis"), ("9aecdb"),
    ("bright_"), ("d6a2e8"),
    ("magenta_"), ("6d214f"),
    ("ending"), ("182c61"),
    ("sasquatch"), ("fc427b"),
    ("pine"), ("bdc581"),
    ("highlighter"), ("82589f"),
    ("bright"), ("cd84f1"),
    ("pretty"), ("ffcccc"),
    ("light__"), ("ff4d4d"),
    ("mandarin"), ("ffaf40"),
    ("unmellow"), ("fffa65"),
    ("light___"), ("c56cf0"),
    ("young"), ("ffb8b8"),
    ("red__"), ("ff3838"),
    ("radiant_"), ("ff9f1a"),
    ("dorn"), ("fff200"),
    ("wintergreen"), ("32ff7e"),
    ("electric"), ("7efff5"),
    ("neon"), ("18dcff"),
    ("light____"), ("7d5fff"),
    ("shadowed"), ("4b4b4b"),
    ("weird"), ("3ae374"),
    ("hammam"), ("67e6dc"),
    ("spiro"), ("17c0eb"),
    ("light"), ("7158e2"),
    ("baltic"), ("3d3d3d"),
    ("sunflower_"), ("ffc312"),
    ("energos"), ("c4e538"),
    ("blue_"), ("12cbc4"),
    ("lavender_"), ("fda7df"),
    ("bara"), ("ed4c67"),
    ("radiant"), ("f79f1f"),
    ("android"), ("a3cb38"),
    ("mediterranean"), ("1289a7"),
    ("lavender___"), ("d980fa"),
    ("very"), ("b53471"),
    ("puffins"), ("ee5a24"),
    ("pixelated"), ("009432"),
    ("merchant"), ("0652dd"),
    ("forgotten"), ("9980fa"),
    ("hollyhock"), ("833471"),
    ("red___"), ("ea2027"),
    ("turkish"), ("006266"),
    ("under"), ("1b1464"),
    ("circumorbital"), ("5758bb"),
    ("magenta__"), ("6f1e51"),
    ("protoss"), ("00a8ff"),
    ("periwinkle"), ("9c88ff"),
    "rise-n-shine":    "fbc531",
    ("download"), ("4cd137"),
    ("seabrook"), ("487eb0"),
    ("vanadyl"), ("0097e6"),
    ("matt"), ("8c7ae6"),
    ("nanohanacha"), ("e1b12c"),
    ("skirret"), ("44bd32"),
    ("naval"), ("40739e"),
    ("nasturcian"), ("e84118"),
    ("lynx"), ("f5f6fa"),
    ("blueberry"), ("7f8fa6"),
    ("mazarine"), ("273c75"),
    ("blue__"), ("353b48"),
    ("harley"), ("c23616"),
    ("hint"), ("dcdde1"),
    ("chain"), ("718093"),
    ("pico"), ("192a56"),
    ("electromagnetic"), ("2f3640"),
