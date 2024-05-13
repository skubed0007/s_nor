use std::{
    fs::File,
    io::Read,
};


pub fn encrypt(file: &str) -> Vec<u8> {
    //println!("{}", "Please Give The File To Be Encrypted!".bold().blue());
    //print!("> ");
    //stdout().flush().unwrap();
    //let mut file = String::new();
    //io::stdin().read_line(&mut file).unwrap();
    let file = file.trim().trim_matches('\"').trim_matches('\'');
    let mut openf = File::open(file).unwrap();
    let mut readcntnt = Vec::new();
    openf.read_to_end(&mut readcntnt).unwrap();
    let mut mapd = String::new();
    for curc in readcntnt {
        #[allow(unreachable_patterns)]
        match curc {
            b'A' => mapd.push_str("100"),
            b'B' => mapd.push_str("101"),
            b'C' => mapd.push_str("102"),
            b'D' => mapd.push_str("103"),
            b'E' => mapd.push_str("104"),
            b'F' => mapd.push_str("105"),
            b'G' => mapd.push_str("106"),
            b'H' => mapd.push_str("107"),
            b'I' => mapd.push_str("108"),
            b'J' => mapd.push_str("109"),
            b'K' => mapd.push_str("110"),
            b'L' => mapd.push_str("111"),
            b'M' => mapd.push_str("112"),
            b'N' => mapd.push_str("113"),
            b'O' => mapd.push_str("114"),
            b'P' => mapd.push_str("115"),
            b'Q' => mapd.push_str("116"),
            b'R' => mapd.push_str("117"),
            b'S' => mapd.push_str("118"),
            b'T' => mapd.push_str("119"),
            b'U' => mapd.push_str("120"),
            b'V' => mapd.push_str("121"),
            b'W' => mapd.push_str("122"),
            b'X' => mapd.push_str("123"),
            b'Y' => mapd.push_str("124"),
            b'Z' => mapd.push_str("125"),
            b'a' => mapd.push_str("200"),
            b'b' => mapd.push_str("201"),
            b'c' => mapd.push_str("202"),
            b'd' => mapd.push_str("203"),
            b'e' => mapd.push_str("204"),
            b'f' => mapd.push_str("205"),
            b'g' => mapd.push_str("206"),
            b'h' => mapd.push_str("207"),
            b'i' => mapd.push_str("208"),
            b'j' => mapd.push_str("209"),
            b'k' => mapd.push_str("210"),
            b'l' => mapd.push_str("211"),
            b'm' => mapd.push_str("212"),
            b'n' => mapd.push_str("213"),
            b'o' => mapd.push_str("214"),
            b'p' => mapd.push_str("215"),
            b'q' => mapd.push_str("216"),
            b'r' => mapd.push_str("217"),
            b's' => mapd.push_str("218"),
            b't' => mapd.push_str("219"),
            b'u' => mapd.push_str("220"),
            b'v' => mapd.push_str("221"),
            b'w' => mapd.push_str("222"),
            b'x' => mapd.push_str("223"),
            b'y' => mapd.push_str("224"),
            b'z' => mapd.push_str("225"),
            b'0' => mapd.push_str("150"),
            b'1' => mapd.push_str("151"),
            b'2' => mapd.push_str("152"),
            b'3' => mapd.push_str("153"),
            b'4' => mapd.push_str("154"),
            b'5' => mapd.push_str("155"),
            b'6' => mapd.push_str("156"),
            b'7' => mapd.push_str("157"),
            b'8' => mapd.push_str("158"),
            b'9' => mapd.push_str("159"),
            b'\n' => mapd.push_str("999"),
            b' ' => mapd.push_str("898"),
            b'!' => mapd.push_str("421"),
            b'"' => mapd.push_str("422"),
            b'#' => mapd.push_str("423"),
            b'$' => mapd.push_str("424"),
            b'%' => mapd.push_str("425"),
            b'&' => mapd.push_str("426"),
            b'\'' => mapd.push_str("427"),
            b'(' => mapd.push_str("428"),
            b')' => mapd.push_str("429"),
            b'*' => mapd.push_str("430"),
            b'+' => mapd.push_str("431"),
            b',' => mapd.push_str("432"),
            b'-' => mapd.push_str("433"),
            b'.' => mapd.push_str("434"),
            b'/' => mapd.push_str("435"),
            b':' => mapd.push_str("436"),
            b';' => mapd.push_str("437"),
            b'<' => mapd.push_str("438"),
            b'=' => mapd.push_str("439"),
            b'>' => mapd.push_str("440"),
            b'?' => mapd.push_str("441"),
            b'@' => mapd.push_str("442"),
            b'[' => mapd.push_str("443"),
            b'\\' => mapd.push_str("444"),
            b']' => mapd.push_str("445"),
            b'^' => mapd.push_str("446"),
            b'_' => mapd.push_str("447"),
            b'`' => mapd.push_str("448"),
            b'{' => mapd.push_str("449"),
            b'|' => mapd.push_str("450"),
            b'}' => mapd.push_str("451"),
            b'~' => mapd.push_str("452"),
            0 => mapd.push_str("567"),
            1 => mapd.push_str("568"),
            2 => mapd.push_str("569"),
            3 => mapd.push_str("570"),
            4 => mapd.push_str("571"),
            5 => mapd.push_str("572"),
            6 => mapd.push_str("573"),
            7 => mapd.push_str("574"),
            8 => mapd.push_str("575"),
            9 => mapd.push_str("576"),
            10 => mapd.push_str("577"),
            11 => mapd.push_str("578"),
            12 => mapd.push_str("579"),
            13 => mapd.push_str("580"),
            14 => mapd.push_str("581"),
            15 => mapd.push_str("582"),
            16 => mapd.push_str("583"),
            17 => mapd.push_str("584"),
            18 => mapd.push_str("585"),
            19 => mapd.push_str("586"),
            20 => mapd.push_str("587"),
            21 => mapd.push_str("588"),
            22 => mapd.push_str("589"),
            23 => mapd.push_str("590"),
            24 => mapd.push_str("591"),
            25 => mapd.push_str("592"),
            26 => mapd.push_str("593"),
            27 => mapd.push_str("594"),
            28 => mapd.push_str("595"),
            29 => mapd.push_str("596"),
            30 => mapd.push_str("597"),
            31 => mapd.push_str("598"),
            32 => mapd.push_str("599"),
            33 => mapd.push_str("600"),
            34 => mapd.push_str("601"),
            35 => mapd.push_str("602"),
            36 => mapd.push_str("603"),
            37 => mapd.push_str("604"),
            38 => mapd.push_str("605"),
            39 => mapd.push_str("606"),
            40 => mapd.push_str("607"),
            41 => mapd.push_str("608"),
            42 => mapd.push_str("609"),
            43 => mapd.push_str("610"),
            44 => mapd.push_str("611"),
            45 => mapd.push_str("612"),
            46 => mapd.push_str("613"),
            47 => mapd.push_str("614"),
            48 => mapd.push_str("615"),
            49 => mapd.push_str("616"),
            50 => mapd.push_str("617"),
            51 => mapd.push_str("618"),
            52 => mapd.push_str("619"),
            53 => mapd.push_str("620"),
            54 => mapd.push_str("621"),
            55 => mapd.push_str("622"),
            56 => mapd.push_str("623"),
            57 => mapd.push_str("624"),
            58 => mapd.push_str("625"),
            59 => mapd.push_str("626"),
            60 => mapd.push_str("627"),
            61 => mapd.push_str("628"),
            62 => mapd.push_str("629"),
            63 => mapd.push_str("630"),
            64 => mapd.push_str("631"),
            65 => mapd.push_str("632"),
            66 => mapd.push_str("633"),
            67 => mapd.push_str("634"),
            68 => mapd.push_str("635"),
            69 => mapd.push_str("636"),
            70 => mapd.push_str("637"),
            71 => mapd.push_str("638"),
            72 => mapd.push_str("639"),
            73 => mapd.push_str("640"),
            74 => mapd.push_str("641"),
            75 => mapd.push_str("642"),
            76 => mapd.push_str("643"),
            77 => mapd.push_str("644"),
            78 => mapd.push_str("645"),
            79 => mapd.push_str("646"),
            80 => mapd.push_str("647"),
            81 => mapd.push_str("648"),
            82 => mapd.push_str("649"),
            83 => mapd.push_str("650"),
            84 => mapd.push_str("651"),
            85 => mapd.push_str("652"),
            86 => mapd.push_str("653"),
            87 => mapd.push_str("654"),
            88 => mapd.push_str("655"),
            89 => mapd.push_str("656"),
            90 => mapd.push_str("657"),
            91 => mapd.push_str("658"),
            92 => mapd.push_str("659"),
            93 => mapd.push_str("660"),
            94 => mapd.push_str("661"),
            95 => mapd.push_str("662"),
            96 => mapd.push_str("663"),
            97 => mapd.push_str("664"),
            98 => mapd.push_str("665"),
            99 => mapd.push_str("666"),
            100 => mapd.push_str("667"),
            101 => mapd.push_str("668"),
            102 => mapd.push_str("669"),
            103 => mapd.push_str("670"),
            104 => mapd.push_str("671"),
            105 => mapd.push_str("672"),
            106 => mapd.push_str("673"),
            107 => mapd.push_str("674"),
            108 => mapd.push_str("675"),
            109 => mapd.push_str("676"),
            110 => mapd.push_str("677"),
            111 => mapd.push_str("678"),
            112 => mapd.push_str("679"),
            113 => mapd.push_str("680"),
            114 => mapd.push_str("681"),
            115 => mapd.push_str("682"),
            116 => mapd.push_str("683"),
            117 => mapd.push_str("684"),
            118 => mapd.push_str("685"),
            119 => mapd.push_str("686"),
            120 => mapd.push_str("687"),
            121 => mapd.push_str("688"),
            122 => mapd.push_str("689"),
            123 => mapd.push_str("690"),
            124 => mapd.push_str("691"),
            125 => mapd.push_str("692"),
            126 => mapd.push_str("693"),
            127 => mapd.push_str("694"),
            128 => mapd.push_str("695"),
            129 => mapd.push_str("696"),
            130 => mapd.push_str("697"),
            131 => mapd.push_str("698"),
            132 => mapd.push_str("699"),
            133 => mapd.push_str("700"),
            134 => mapd.push_str("701"),
            135 => mapd.push_str("702"),
            136 => mapd.push_str("703"),
            137 => mapd.push_str("704"),
            138 => mapd.push_str("705"),
            139 => mapd.push_str("706"),
            140 => mapd.push_str("707"),
            141 => mapd.push_str("708"),
            142 => mapd.push_str("709"),
            143 => mapd.push_str("710"),
            144 => mapd.push_str("711"),
            145 => mapd.push_str("712"),
            146 => mapd.push_str("713"),
            147 => mapd.push_str("714"),
            148 => mapd.push_str("715"),
            149 => mapd.push_str("716"),
            150 => mapd.push_str("717"),
            151 => mapd.push_str("718"),
            152 => mapd.push_str("719"),
            153 => mapd.push_str("720"),
            154 => mapd.push_str("721"),
            155 => mapd.push_str("722"),
            156 => mapd.push_str("723"),
            157 => mapd.push_str("724"),
            158 => mapd.push_str("725"),
            159 => mapd.push_str("726"),
            160 => mapd.push_str("727"),
            161 => mapd.push_str("728"),
            162 => mapd.push_str("729"),
            163 => mapd.push_str("730"),
            164 => mapd.push_str("731"),
            165 => mapd.push_str("732"),
            166 => mapd.push_str("733"),
            167 => mapd.push_str("734"),
            168 => mapd.push_str("735"),
            169 => mapd.push_str("736"),
            170 => mapd.push_str("737"),
            171 => mapd.push_str("738"),
            172 => mapd.push_str("739"),
            173 => mapd.push_str("740"),
            174 => mapd.push_str("741"),
            175 => mapd.push_str("742"),
            176 => mapd.push_str("743"),
            177 => mapd.push_str("744"),
            178 => mapd.push_str("745"),
            179 => mapd.push_str("746"),
            180 => mapd.push_str("747"),
            181 => mapd.push_str("748"),
            182 => mapd.push_str("749"),
            183 => mapd.push_str("750"),
            184 => mapd.push_str("751"),
            185 => mapd.push_str("752"),
            186 => mapd.push_str("753"),
            187 => mapd.push_str("754"),
            188 => mapd.push_str("755"),
            189 => mapd.push_str("756"),
            190 => mapd.push_str("757"),
            191 => mapd.push_str("758"),
            192 => mapd.push_str("759"),
            193 => mapd.push_str("760"),
            194 => mapd.push_str("761"),
            195 => mapd.push_str("762"),
            196 => mapd.push_str("763"),
            197 => mapd.push_str("764"),
            198 => mapd.push_str("765"),
            199 => mapd.push_str("766"),
            200 => mapd.push_str("767"),
            201 => mapd.push_str("768"),
            202 => mapd.push_str("769"),
            203 => mapd.push_str("770"),
            204 => mapd.push_str("771"),
            205 => mapd.push_str("772"),
            206 => mapd.push_str("773"),
            207 => mapd.push_str("774"),
            208 => mapd.push_str("775"),
            209 => mapd.push_str("776"),
            210 => mapd.push_str("777"),
            211 => mapd.push_str("778"),
            212 => mapd.push_str("779"),
            213 => mapd.push_str("780"),
            214 => mapd.push_str("781"),
            215 => mapd.push_str("782"),
            216 => mapd.push_str("783"),
            217 => mapd.push_str("784"),
            218 => mapd.push_str("785"),
            219 => mapd.push_str("786"),
            220 => mapd.push_str("787"),
            221 => mapd.push_str("788"),
            222 => mapd.push_str("789"),
            223 => mapd.push_str("790"),
            224 => mapd.push_str("791"),
            225 => mapd.push_str("792"),
            226 => mapd.push_str("793"),
            227 => mapd.push_str("794"),
            228 => mapd.push_str("795"),
            229 => mapd.push_str("796"),
            230 => mapd.push_str("797"),
            231 => mapd.push_str("798"),
            232 => mapd.push_str("799"),
            233 => mapd.push_str("800"),
            234 => mapd.push_str("801"),
            235 => mapd.push_str("802"),
            236 => mapd.push_str("803"),
            237 => mapd.push_str("804"),
            238 => mapd.push_str("805"),
            239 => mapd.push_str("806"),
            240 => mapd.push_str("807"),
            241 => mapd.push_str("808"),
            242 => mapd.push_str("809"),
            243 => mapd.push_str("810"),
            244 => mapd.push_str("811"),
            245 => mapd.push_str("812"),
            246 => mapd.push_str("813"),
            247 => mapd.push_str("814"),
            248 => mapd.push_str("815"),
            249 => mapd.push_str("816"),
            250 => mapd.push_str("817"),
            251 => mapd.push_str("818"),
            252 => mapd.push_str("819"),
            253 => mapd.push_str("820"),
            254 => mapd.push_str("821"),
            255 => mapd.push_str("822"),

            _ => {}
        }
    }
    let mapppd = mapd.clone();
    let bid = mapppd.to_owned().clone();
    let mappd = bid.as_bytes();
    return mappd.to_vec();
    //println!("{}", mapd);
}

pub fn decrypt(file: &str) -> Vec<u8> {
    let mut decrypted_data: Vec<u8> = Vec::new();
    let encrypted_file = file.trim().trim_matches('\"').trim_matches('\'');
    let mut openf = File::open(encrypted_file).unwrap();
    let mut encrypted_data = Vec::new();
    openf.read_to_end(&mut encrypted_data).unwrap();
    let mut parts = encrypted_data.splitn(2, |&x| {
        x == b'|' && encrypted_data.get(1) == Some(&b'|') && encrypted_data.get(2) == Some(&b'|')
    });
    let passworrd = parts.next().unwrap_or(&[]);
    let passworrd = String::from_utf8_lossy(passworrd);
    let mut passwordndencdat = passworrd.split("-_-");
    let encrypted_data = passwordndencdat.next().unwrap_or("").trim().as_bytes();
    //let encrypted_data = parts.next().unwrap_or(&[]);
    for chunk in encrypted_data.chunks(3) {
        if let [first, second, third] = chunk {
            let num_str = format!("{}{}{}", *first as char, *second as char, *third as char);
            let num: u16 = num_str.parse().unwrap();
            match num {
                100 => decrypted_data.push(b'A'),
                101 => decrypted_data.push(b'B'),
                102 => decrypted_data.push(b'C'),
                103 => decrypted_data.push(b'D'),
                104 => decrypted_data.push(b'E'),
                105 => decrypted_data.push(b'F'),
                106 => decrypted_data.push(b'G'),
                107 => decrypted_data.push(b'H'),
                108 => decrypted_data.push(b'I'),
                109 => decrypted_data.push(b'J'),
                110 => decrypted_data.push(b'K'),
                111 => decrypted_data.push(b'L'),
                112 => decrypted_data.push(b'M'),
                113 => decrypted_data.push(b'N'),
                114 => decrypted_data.push(b'O'),
                115 => decrypted_data.push(b'P'),
                116 => decrypted_data.push(b'Q'),
                117 => decrypted_data.push(b'R'),
                118 => decrypted_data.push(b'S'),
                119 => decrypted_data.push(b'T'),
                120 => decrypted_data.push(b'U'),
                121 => decrypted_data.push(b'V'),
                122 => decrypted_data.push(b'W'),
                123 => decrypted_data.push(b'X'),
                124 => decrypted_data.push(b'Y'),
                125 => decrypted_data.push(b'Z'),
                200 => decrypted_data.push(b'a'),
                201 => decrypted_data.push(b'b'),
                202 => decrypted_data.push(b'c'),
                203 => decrypted_data.push(b'd'),
                204 => decrypted_data.push(b'e'),
                205 => decrypted_data.push(b'f'),
                206 => decrypted_data.push(b'g'),
                207 => decrypted_data.push(b'h'),
                208 => decrypted_data.push(b'i'),
                209 => decrypted_data.push(b'j'),
                210 => decrypted_data.push(b'k'),
                211 => decrypted_data.push(b'l'),
                212 => decrypted_data.push(b'm'),
                213 => decrypted_data.push(b'n'),
                214 => decrypted_data.push(b'o'),
                215 => decrypted_data.push(b'p'),
                216 => decrypted_data.push(b'q'),
                217 => decrypted_data.push(b'r'),
                218 => decrypted_data.push(b's'),
                219 => decrypted_data.push(b't'),
                220 => decrypted_data.push(b'u'),
                221 => decrypted_data.push(b'v'),
                222 => decrypted_data.push(b'w'),
                223 => decrypted_data.push(b'x'),
                224 => decrypted_data.push(b'y'),
                225 => decrypted_data.push(b'z'),
                150 => decrypted_data.push(b'0'),
                151 => decrypted_data.push(b'1'),
                152 => decrypted_data.push(b'2'),
                153 => decrypted_data.push(b'3'),
                154 => decrypted_data.push(b'4'),
                155 => decrypted_data.push(b'5'),
                156 => decrypted_data.push(b'6'),
                157 => decrypted_data.push(b'7'),
                158 => decrypted_data.push(b'8'),
                159 => decrypted_data.push(b'9'),
                999 => decrypted_data.push(b'\n'),
                898 => decrypted_data.push(b' '),
                421 => decrypted_data.push(b'!'),
                422 => decrypted_data.push(b'"'),
                423 => decrypted_data.push(b'#'),
                424 => decrypted_data.push(b'$'),
                425 => decrypted_data.push(b'%'),
                426 => decrypted_data.push(b'&'),
                427 => decrypted_data.push(b'\''),
                428 => decrypted_data.push(b'('),
                429 => decrypted_data.push(b')'),
                430 => decrypted_data.push(b'*'),
                431 => decrypted_data.push(b'+'),
                432 => decrypted_data.push(b','),
                433 => decrypted_data.push(b'-'),
                434 => decrypted_data.push(b'.'),
                435 => decrypted_data.push(b'/'),
                436 => decrypted_data.push(b':'),
                437 => decrypted_data.push(b';'),
                438 => decrypted_data.push(b'<'),
                439 => decrypted_data.push(b'='),
                440 => decrypted_data.push(b'>'),
                441 => decrypted_data.push(b'?'),
                442 => decrypted_data.push(b'@'),
                443 => decrypted_data.push(b'['),
                444 => decrypted_data.push(b'\\'),
                445 => decrypted_data.push(b']'),
                446 => decrypted_data.push(b'^'),
                447 => decrypted_data.push(b'_'),
                448 => decrypted_data.push(b'`'),
                449 => decrypted_data.push(b'{'),
                450 => decrypted_data.push(b'|'),
                451 => decrypted_data.push(b'}'),
                452 => decrypted_data.push(b'~'),
                567 => decrypted_data.push(0),
                568 => decrypted_data.push(1),
                569 => decrypted_data.push(2),
                570 => decrypted_data.push(3),
                571 => decrypted_data.push(4),
                572 => decrypted_data.push(5),
                573 => decrypted_data.push(6),
                574 => decrypted_data.push(7),
                575 => decrypted_data.push(8),
                576 => decrypted_data.push(9),
                577 => decrypted_data.push(10),
                578 => decrypted_data.push(11),
                579 => decrypted_data.push(12),
                580 => decrypted_data.push(13),
                581 => decrypted_data.push(14),
                582 => decrypted_data.push(15),
                583 => decrypted_data.push(16),
                584 => decrypted_data.push(17),
                585 => decrypted_data.push(18),
                586 => decrypted_data.push(19),
                587 => decrypted_data.push(20),
                588 => decrypted_data.push(21),
                589 => decrypted_data.push(22),
                590 => decrypted_data.push(23),
                591 => decrypted_data.push(24),
                592 => decrypted_data.push(25),
                593 => decrypted_data.push(26),
                594 => decrypted_data.push(27),
                595 => decrypted_data.push(28),
                596 => decrypted_data.push(29),
                597 => decrypted_data.push(30),
                598 => decrypted_data.push(31),
                599 => decrypted_data.push(32),
                600 => decrypted_data.push(33),
                601 => decrypted_data.push(34),
                602 => decrypted_data.push(35),
                603 => decrypted_data.push(36),
                604 => decrypted_data.push(37),
                605 => decrypted_data.push(38),
                606 => decrypted_data.push(39),
                607 => decrypted_data.push(40),
                608 => decrypted_data.push(41),
                609 => decrypted_data.push(42),
                610 => decrypted_data.push(43),
                611 => decrypted_data.push(44),
                612 => decrypted_data.push(45),
                613 => decrypted_data.push(46),
                614 => decrypted_data.push(47),
                615 => decrypted_data.push(48),
                616 => decrypted_data.push(49),
                617 => decrypted_data.push(50),
                618 => decrypted_data.push(51),
                619 => decrypted_data.push(52),
                620 => decrypted_data.push(53),
                621 => decrypted_data.push(54),
                622 => decrypted_data.push(55),
                623 => decrypted_data.push(56),
                624 => decrypted_data.push(57),
                625 => decrypted_data.push(58),
                626 => decrypted_data.push(59),
                627 => decrypted_data.push(60),
                628 => decrypted_data.push(61),
                629 => decrypted_data.push(62),
                630 => decrypted_data.push(63),
                631 => decrypted_data.push(64),
                632 => decrypted_data.push(65),
                633 => decrypted_data.push(66),
                634 => decrypted_data.push(67),
                635 => decrypted_data.push(68),
                636 => decrypted_data.push(69),
                637 => decrypted_data.push(70),
                638 => decrypted_data.push(71),
                639 => decrypted_data.push(72),
                640 => decrypted_data.push(73),
                641 => decrypted_data.push(74),
                642 => decrypted_data.push(75),
                643 => decrypted_data.push(76),
                644 => decrypted_data.push(77),
                645 => decrypted_data.push(78),
                646 => decrypted_data.push(79),
                647 => decrypted_data.push(80),
                648 => decrypted_data.push(81),
                649 => decrypted_data.push(82),
                650 => decrypted_data.push(83),
                651 => decrypted_data.push(84),
                652 => decrypted_data.push(85),
                653 => decrypted_data.push(86),
                654 => decrypted_data.push(87),
                655 => decrypted_data.push(88),
                656 => decrypted_data.push(89),
                657 => decrypted_data.push(90),
                658 => decrypted_data.push(91),
                659 => decrypted_data.push(92),
                660 => decrypted_data.push(93),
                661 => decrypted_data.push(94),
                662 => decrypted_data.push(95),
                663 => decrypted_data.push(96),
                664 => decrypted_data.push(97),
                665 => decrypted_data.push(98),
                666 => decrypted_data.push(99),
                667 => decrypted_data.push(100),
                668 => decrypted_data.push(101),
                669 => decrypted_data.push(102),
                670 => decrypted_data.push(103),
                671 => decrypted_data.push(104),
                672 => decrypted_data.push(105),
                673 => decrypted_data.push(106),
                674 => decrypted_data.push(107),
                675 => decrypted_data.push(108),
                676 => decrypted_data.push(109),
                677 => decrypted_data.push(110),
                678 => decrypted_data.push(111),
                679 => decrypted_data.push(112),
                680 => decrypted_data.push(113),
                681 => decrypted_data.push(114),
                682 => decrypted_data.push(115),
                683 => decrypted_data.push(116),
                684 => decrypted_data.push(117),
                685 => decrypted_data.push(118),
                686 => decrypted_data.push(119),
                687 => decrypted_data.push(120),
                688 => decrypted_data.push(121),
                689 => decrypted_data.push(122),
                690 => decrypted_data.push(123),
                691 => decrypted_data.push(124),
                692 => decrypted_data.push(125),
                693 => decrypted_data.push(126),
                694 => decrypted_data.push(127),
                695 => decrypted_data.push(128),
                696 => decrypted_data.push(129),
                697 => decrypted_data.push(130),
                698 => decrypted_data.push(131),
                699 => decrypted_data.push(132),
                700 => decrypted_data.push(133),
                701 => decrypted_data.push(134),
                702 => decrypted_data.push(135),
                703 => decrypted_data.push(136),
                704 => decrypted_data.push(137),
                705 => decrypted_data.push(138),
                706 => decrypted_data.push(139),
                707 => decrypted_data.push(140),
                708 => decrypted_data.push(141),
                709 => decrypted_data.push(142),
                710 => decrypted_data.push(143),
                711 => decrypted_data.push(144),
                712 => decrypted_data.push(145),
                713 => decrypted_data.push(146),
                714 => decrypted_data.push(147),
                715 => decrypted_data.push(148),
                716 => decrypted_data.push(149),
                717 => decrypted_data.push(150),
                718 => decrypted_data.push(151),
                719 => decrypted_data.push(152),
                720 => decrypted_data.push(153),
                721 => decrypted_data.push(154),
                722 => decrypted_data.push(155),
                723 => decrypted_data.push(156),
                724 => decrypted_data.push(157),
                725 => decrypted_data.push(158),
                726 => decrypted_data.push(159),
                727 => decrypted_data.push(160),
                728 => decrypted_data.push(161),
                729 => decrypted_data.push(162),
                730 => decrypted_data.push(163),
                731 => decrypted_data.push(164),
                732 => decrypted_data.push(165),
                733 => decrypted_data.push(166),
                734 => decrypted_data.push(167),
                735 => decrypted_data.push(168),
                736 => decrypted_data.push(169),
                737 => decrypted_data.push(170),
                738 => decrypted_data.push(171),
                739 => decrypted_data.push(172),
                740 => decrypted_data.push(173),
                741 => decrypted_data.push(174),
                742 => decrypted_data.push(175),
                743 => decrypted_data.push(176),
                744 => decrypted_data.push(177),
                745 => decrypted_data.push(178),
                746 => decrypted_data.push(179),
                747 => decrypted_data.push(180),
                748 => decrypted_data.push(181),
                749 => decrypted_data.push(182),
                750 => decrypted_data.push(183),
                751 => decrypted_data.push(184),
                752 => decrypted_data.push(185),
                753 => decrypted_data.push(186),
                754 => decrypted_data.push(187),
                755 => decrypted_data.push(188),
                756 => decrypted_data.push(189),
                757 => decrypted_data.push(190),
                758 => decrypted_data.push(191),
                759 => decrypted_data.push(192),
                760 => decrypted_data.push(193),
                761 => decrypted_data.push(194),
                762 => decrypted_data.push(195),
                763 => decrypted_data.push(196),
                764 => decrypted_data.push(197),
                765 => decrypted_data.push(198),
                766 => decrypted_data.push(199),
                767 => decrypted_data.push(200),
                768 => decrypted_data.push(201),
                769 => decrypted_data.push(202),
                770 => decrypted_data.push(203),
                771 => decrypted_data.push(204),
                772 => decrypted_data.push(205),
                773 => decrypted_data.push(206),
                774 => decrypted_data.push(207),
                775 => decrypted_data.push(208),
                776 => decrypted_data.push(209),
                777 => decrypted_data.push(210),
                778 => decrypted_data.push(211),
                779 => decrypted_data.push(212),
                780 => decrypted_data.push(213),
                781 => decrypted_data.push(214),
                782 => decrypted_data.push(215),
                783 => decrypted_data.push(216),
                784 => decrypted_data.push(217),
                785 => decrypted_data.push(218),
                786 => decrypted_data.push(219),
                787 => decrypted_data.push(220),
                788 => decrypted_data.push(221),
                789 => decrypted_data.push(222),
                790 => decrypted_data.push(223),
                791 => decrypted_data.push(224),
                792 => decrypted_data.push(225),
                793 => decrypted_data.push(226),
                794 => decrypted_data.push(227),
                795 => decrypted_data.push(228),
                796 => decrypted_data.push(229),
                797 => decrypted_data.push(230),
                798 => decrypted_data.push(231),
                799 => decrypted_data.push(232),
                800 => decrypted_data.push(233),
                801 => decrypted_data.push(234),
                802 => decrypted_data.push(235),
                803 => decrypted_data.push(236),
                804 => decrypted_data.push(237),
                805 => decrypted_data.push(238),
                806 => decrypted_data.push(239),
                807 => decrypted_data.push(240),
                808 => decrypted_data.push(241),
                809 => decrypted_data.push(242),
                810 => decrypted_data.push(243),
                811 => decrypted_data.push(244),
                812 => decrypted_data.push(245),
                813 => decrypted_data.push(246),
                814 => decrypted_data.push(247),
                815 => decrypted_data.push(248),
                816 => decrypted_data.push(249),
                817 => decrypted_data.push(250),
                818 => decrypted_data.push(251),
                819 => decrypted_data.push(252),
                820 => decrypted_data.push(253),
                821 => decrypted_data.push(254),
                822 => decrypted_data.push(255),

                _ => {}
            }
        }
    }
    let instr = String::from_utf8_lossy(&decrypted_data);
    return instr.as_bytes().to_vec();
}

#[cfg(test)]
mod tests {
    use crate::{decrypt, encrypt};

    #[test]
    fn tst_en() {
        let encdat = encrypt("./tst_enc.txt");
        println!("{:?}",encdat);
    }
    #[test]
    fn tst_den() {
        let dencdat = decrypt("./1_dec_tst.neko");
        if dencdat == 1.to_string().as_bytes().to_vec(){
            println!("EERRR");
        }
        println!("{:?}",dencdat);
    }
    
}
