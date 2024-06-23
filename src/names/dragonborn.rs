use super::*;

pub const DRAGONBORN_MALE_NAMES: [&'static str; 101] = [
    "Arjhan", "Balasar", "Chalden", "Darvax", "Ervan", "Fenthos", "Galdar", 
    "Heskan", "Ildrex", "Jalassar", "Kaethar", "Lorthas", "Marduk", "Nadarr", 
    "Orikhan", "Palthor", "Qelroth", "Rathian", "Sarkhan", "Torinn", 
    "Uldren", "Vathak", "Wyrn", "Xavhorn", "Yordrax", "Zarnith", "Arthak", 
    "Belthar", "Carnyx", "Dravak", "Eldrin", "Falkor", "Garash", "Harkon", 
    "Inthorn", "Jorath", "Krivak", "Mythos", "Nithrax", "Othran", 
    "Pyrros", "Qorvax", "Rivorn", "Sorath", "Tarhun", "Ulthar", "Vorkhan", 
    "Wyrmorn", "Xalthar", "Yrros", "Zorvath", "Aldrik", "Bryndar", "Corthak", 
    "Drakar", "Erthon", "Fayrith", "Gorash", "Haldir", "Isharn", "Jorlan", 
    "Kalthar", "Lorthos", "Myrdak", "Nalkor", "Orinash", "Pyrnax", "Qalthos", 
    "Rithar", "Sordran", "Tormak", "Ulthor", "Vorash", "Wrythak", "Xaroth", 
    "Yrnax", "Zarvath", "Aldros", "Brathar", "Caldor", "Drathen", "Eryndor", 
    "Fandor", "Gorlan", "Harkoth", "Ilthar", "Jorvan", "Karnyx", "Lorash", 
    "Mythran", "Nalvor", "Orvax", "Pyrrhos", "Qildor", "Rythorn", "Sarnor", 
    "Tharvax", "Ulnar", "Vornax", "Wyrndor", "Xarvath"
];

pub const DRAGONBORN_FEMALE_NAMES: [&'static str; 101] = [
    "Arjhana", "Balaath", "Corthira", "Darthyn", "Elaris", "Fenthara", "Galyth", 
    "Heskara", "Ildrynn", "Jalassa", "Kaethara", "Lorthana", "Marda", "Nadarya", 
    "Orithira", "Palthara", "Qelrissa", "Rathina", "Sarkara", "Torrina", 
    "Uldra", "Vathana", "Wyrnara", "Xavhira", "Yordrynn", "Zarnitha", 
    "Arthira", "Belthara", "Carnyxia", "Dravara", "Eldryss", "Falkira", "Garasha", 
    "Harkara", "Inthira", "Joratha", "Krivara", "Lorana", "Mythara", "Nithra", 
    "Othriya", "Pyrrissa", "Qorvyth", "Rivara", "Soranna", "Tarhira", "Ulthara", 
    "Vorkara", "Xalthira", "Yrissa", "Zorvatha", "Aldra", "Bryndara", 
    "Corthina", "Drakana", "Erthara", "Fayrith", "Gorana", "Haldira", "Ishara", 
    "Jorla", "Kalthina", "Lorthya", "Myrda", "Nalkara", "Orinya", "Pyrnara", 
    "Qaltha", "Rithara", "Sordra", "Tormara", "Ulthora", "Vorisha", "Wrytha", 
    "Xarisha", "Yrnara", "Zarvana", "Aldara", "Brathara", "Caldora", "Drathina", 
    "Eryndara", "Fandora", "Gorala", "Harkira", "Ilthara", "Jorvya", "Karnyxia", 
    "Lora", "Mythrina", "Nalvora", "Orvaya", "Pyrrina", "Qildra", "Rythara", 
    "Sarnara", "Tharvyna", "Ulna", "Vorna", "Wyrndra", "Xarvyna"
];

pub const DRAGONBORN_SURNAMES: [&'static str; 104] = [
    "Ardros", "Belthrax", "Corvath", "Drek'thar", "Exaros", "Flametongue", 
    "Gythrex", "Havocclaw", "Irontail", "Jorakas", "Kaz'garoth", "Lor'athor", 
    "Morthos", "Nar'shak", "Othrys", "Pyreclaw", "Qel'zin", "Razorthorn", 
    "Sarthak", "Thundertail", "Uldrex", "Vor'zath", "Wyrmscale", "Xal'atath", 
    "Y'shaarj", "Zar'jira", "Aldrexus", "Brimstone", "Cindertail", "Drakonius", 
    "Emberfang", "Firebrand", "Goreclaw", "Hawkscream", "Ignatius", "Jadefire", 
    "Kor'vas", "Lavalash", "Moltenfang", "Nyxathor", "Obsidian", "Pyrax", 
    "Quel'darim", "Ragehorn", "Scalescar", "Stormclaw", "Tyranthraxus", 
    "Umbrask", "Vex'ahlia", "Wrathclaw", "Xyn'thas", "Ymirjar", "Zephyrion", 
    "Ashenclaw", "Bloodfist", "Crimsontalon", "Dreadfang", "Emberclaw", 
    "Frostbane", "Gloomwing", "Hollowscale", "Infernoth", "Jormungar", 
    "Krag'har", "Lunarion", "Mistwalker", "Nightshade", "Onyxia", "Pyrestrike", 
    "Quel'saris", "Ravenshadow", "Shadowflame", "Thundercrash", "Uldorak", 
    "Viperfang", "Winterwrath", "Xal'kazar", "Ys'thra", "Zar'jhan", "Auraxius", 
    "Blazewing", "Crimsonclaw", "Deathwing", "Eclipse", "Frostscale", 
    "Grimtotem", "Horncaller", "Ironclaw", "Jadefang", "Krakenscale", 
    "Lorekeeper", "Mournwings", "Netherdrake", "Oathbreaker", "Pyroclasm", 
    "Quel'dorei", "Razorwing", "Serpentstrike", "Twilight", "Vengeclaw", 
    "Wyrmbreaker", "Xiraxis", "Yggdrasil", "Zephyros"
];