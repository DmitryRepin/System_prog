FROM amazonlinux:2

COPY scr.sh .

RUN chmod ugo+x /scr.sh

CMD ./scr.sh
