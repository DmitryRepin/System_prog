#include <stdio.h>
#include <string.h>

const char incorrect_pas[] = "───────▄▀▀▀▀▀▀▀▀▀▀▄▄\n────▄▀▀░░░░░░░░░░░░░▀▄\n──▄▀░░░░░░░░░░░░░░░░░░▀▄\n──█░░░░░░░░░░░░░░░░░░░░░▀▄\n─▐▌░░░░░░░░▄▄▄▄▄▄▄░░░░░░░▐▌\n─█░░░░░░░░░░░▄▄▄▄░░▀▀▀▀▀░░█\n▐▌░░░░░░░▀▀▀▀░░░░░▀▀▀▀▀░░░▐▌\n█░░░░░░░░░▄▄▀▀▀▀▀░░░░▀▀▀▀▄░█\n█░░░░░░░░░░░░░░░░▀░░░▐░░░░░▐▌\n▐▌░░░░░░░░░▐▀▀██▄░░░░░░▄▄▄░▐▌\n─█░░░░░░░░░░░▀▀▀░░░░░░▀▀██░░█\n─▐▌░░░░▄░░░░░░░░░░░░░▌░░░░░░█\n──▐▌░░▐░░░░░░░░░░░░░░▀▄░░░░░█\n───█░░░▌░░░░░░░░▐▀░░░░▄▀░░░▐▌\n───▐▌░░▀▄░░░░░░░░▀░▀░▀▀░░░▄▀\n───▐▌░░▐▀▄░░░░░░░░░░░░░░░░█\n───▐▌░░░▌░▀▄░░░░▀▀▀▀▀▀░░░█\n───█░░░▀░░░░▀▄░░░░░░░░░░▄▀\n──▐▌░░░░░░░░░░▀▄░░░░░░▄▀\n─▄▀░░░▄▀░░░░░░░░▀▀▀▀█▀\n▀░░░▄▀░░░░░░░░░░▀░░░▀▀▀▀▄▄▄▄▄\n";
const char correct_pas[] = "                                           ...::::::::...                                           \n                                  .:^!?J5PGGBBBBBBBBBBBBGGP5J?!^:.                                  \n                             .^7JPB####BBGGPPPPPPPPPPPPPPGGBB####BPJ7^.                             \n                         .~?PB##BGGPP55PPPGGGGBBBBBBBBBGGGPPPP5PPGGB##BP?~.                         \n                      :!5B##BGP55PGGBB##########&#############BBGGP55PGB##B57:                      \n                   .!5##BGP5PPGB###########&&###GB####&&##&&&#######BGPP5PGB##5!.                   \n                 ^YB&#GP5PGB#&&BYP&##&#BB&&5?PJ^JG?~P5!J&#GPGB&#&&######BGP55G#&BY^                 \n               ~5##GP5PGB###B5##5?&#B7P5:?&!:BY:G@! B&!.B~:5G^5BJG#########BGP5PG##5~               \n             ~P&#G55PB#&&B#P?.!Y#Y5&B??GJ Y5~77?J5!~BB~~P 7BPJGJ J5!B###&&&&&#BP55G#&P~             \n           :Y&#G55PB###G?:JB&J77^!^B&J.YP5YY~Y5575&###BB#B5PPPY Y&#B##&#P?7?YP#&BP55G#&5:           \n          7#&BP5PB######B57^~J7&BY~Y&#BB#&&#BBBB#&########&###BPG####&G^ !PBBBJG##BP5PB&#7          \n        .5&#P55G####&####&&B5!.J&&&#########BBBBBBBBBBBB#######&#####&!:G&#B&&!J####BP5P#&5.        \n       ^G&BP5P####P?YG######&&BG######BBGGGBBBBBBPPGPGGGBBGGBB#######&P?B#G~7GB&&#####P5PB&B^       \n      ^#&G55G####JB#BB##PB#########BGGBB#&#PJPY!~YY^^:~7!?###BGGB#####&#GG!~PGP~G&#####G55G&#~      \n     ^#&G55G###&G.5#&&&&B?&#####BGGB##&#P7:  ^JPGP5YGG55775!75##BGGB#####&GY?~~?YY######G55G&#^     \n    :B&G55G#####&J::!J55JP&###BGGB###&P^      ^?JYPBG~        ^P&##GGB####&G7G&B?~G######G55G&B:    \n    P&B55G##&GYB#&#PJ??5B&###GG#######?           .!P?         ?&####GG######&?^7JYY######G55B&P    \n   7&#P5P###&J.^~~!7JJP&###BPB#####&G:5J^         ..:5?      :75^G&###BPB#####7PBGY~B&#####P5P#&7   \n  .B&G55B###&#G55555Y~####BPB######&G  #J  ^^^:  .Y~~^:.:^.  5#. G&####BPB#####Y7~!YYG&####B55G&B.  \n  !&#P5P####PYYJ?!^75B####PB########&7~P~   .7J7:.:  :!77!   !G^7&######BP#####&7!777P&#####P5P#&7  \n  5&B55G###B~~!?JY5PY####PG###########JPBPJ!^.^!Y7  7Y!^:^~7YGYY#########GP#####7YGGP?B#####G55B&5  \n .B&G55B###BB#B#BGG7J&###P###########B5#B&&&&B5G~?PG7:YYG#&&&##J#&########PB###&P~7?JJB#####B55G&B. \n :#&G55#####J~~7#&&B####BP###GG&GG#&!:.~:JPB#&&#G?P5JB&&&##&5^7:~YBG&GG###PG#&?!7!??J7P&#####P5P##: \n :##P5P###&P7BB5J&&#P###GPGGGPPGPG##5^  ..:?P57~ ~P5^~?JP5?!^^~ ^5GPGPPGGGPG#&5PJ75P5!G&#####P5P##^ \n :#&G55###&G!?77:!!~!&##BP###GG&GG##&#GGPPG#Y!7~7&&&#!?~!GB555BG#&GG&GG###PG##&&5^???^P&#####P5P##: \n .B&G55B####!Y55PPPGY####P############&&G^~7??7^^7^7J!!!Y7?:7&#&##########PB#####G5Y5G######B55G&B. \n  5&B55G#####&&&&&&&&####PG#############&BPY?.Y777!!^?5Y^?JP#############GP#######&&&#######G55B&5  \n  !&#P5P######&&&B7&&&&###PB#############&?J5Y~^J!?Y^? !JP5B&###########BP#####&&&B7#&&&####P5P#&7  \n  .B&G55B#####PY5! ?55G###BPB#############7~P?PYP5J5JJJ55J^P&##########BPB#####PY5! ?55P###B55G&B.  \n   7&#P5P#####GY:   ~5B####BPB############&BY?J75:B7?5?J?YB###########BPB######GY:   ^5B###P5P#&7   \n    P&B55G####&B^!J^!&###&&&#GG#############&#?^:^^!777G#&###########GG########&B^~J^~####G55B&P    \n    :B&G55G####GB&&#GB&&#57Y&#BGG##############YJJJJJ?G&###########GGB####&&#&&&GG&&#GB##G55G&B:    \n     ^#&G55G########&#P?~?5!7###BGGB###########BBPPBG#&#########BGGB####&&G!!75#&&###&##G55G&#^     \n      ~#&G55G#######J~!5G?!YG&#####BGGBB#######B#GP#B#######BBGGB#####&&PGJJ5:7^?######G55G&#~      \n       ^G&BP5P#####BYBBJ~JY 7YB&&#####BBGGGBBBB##GP##BBBBGGGBB#####&&&G!^7~Y5~7~P#####P5PB&B^       \n        .5&#P55G####B?^.?7~5?^PJYG#&&#######BBBBBBBBBBBB#######&##&P77!!~JP^^BG#&###BP5P#&5.        \n          7#&BP5PB#&5JBGYP&5 !!Y!:7!5#B#&#&&&####GPPB&#&&&#&##BG#7Y?.?~5P?^.J#&###BP5PG&#7          \n           :5&#G55PB&&#&#PJ:7P#7^B7:P^!7!BG5P####.!J.G#P5GB77?5:J? ?Y:?!!&BJ!B&#BP55G#&5:           \n             ~P&#G55PB#&BYYB&&5~B7^B!^#^~#??:7&##:!?.B577 5^~&&G.~G:J#PP###&##BP55G#&P~             \n               ~5##GP5PGB&&&###&#Y#P:B?.B!!J Y&##:!G:YB^Y:~B.J&&B!G#B#&&###BGP5PG##P~               \n                 ^YB&BG55PGB#####&####5G#7!7^B###^~J:5B^77!#P5###&##&###BGP55GB&BY^                 \n                   .75##BGP5PPGB######&&#&#######BGGB&##B#&#&&######BGPP5PGB##P7.                   \n                      :75B##BGP55PGGBB###########&&&&##########BGGP55PGB##B57:                      \n                         .~?PB##BGGPP5PPPPGGGBBBBBBBBBBGGGPPPP5PPGGB##BP?~.                         \n                             .^7YPB####BBGGPPPPPPPPPPPPPPGGBB####BPY7^.                             \n                                  .:^!?J5PGGBBBBBBBBBBBBGGP5J?!~:.                                  \n                                           ..::::^^::::..                                           \n";

void secure_check(const char * name)
{
	char buff[6];
	char current_pass[] = "tusur";

	printf("%s, enter your pass: ", name);
	scanf("%s", buff);

	if (strncmp(buff, current_pass, sizeof(current_pass) - 1) == 0)
	{
		printf("\ncorrect password\n\n");
		printf("%s", correct_pas);
	}
	else
	{
		printf("incorrect password\n\n");
		printf("\n%s", incorrect_pas);
	}
}

int top_secret_function()
{
	printf("Access success!\n");
	return 0;
}

int main()
{
	char buff[10] = {};
	printf("Enter your name: ");
	scanf("%s", buff);
	secure_check(buff);
	return 0;
}