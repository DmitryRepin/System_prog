#!/bin/bash

echo "Дмитрий Репин 729-1"
echo "Поиск файла и вывод информации о нем"

while :
do 
echo "Введите имя файла"

read FILENAME

FOUNDFILE=$(find / -maxdepth 1 -type f -name $FILENAME 2>/dev/null)

if [ "$FOUNDFILE" == "" ]; then
	echo "Файл не найден"
else
	stat --printf "Полное имя файла: %n\nТип файла: %F\nРазмер файла: %s\nВладелец файла: %U\nПрава доступа: %A\nДата создания: %y\n" $FOUNDFILE
fi

echo "Хотите продолжить Y/n?"
read ANSWER

if [ "$ANSWER" == "n" ]; then
	break
fi

done
