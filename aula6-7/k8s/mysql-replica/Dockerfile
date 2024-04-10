FROM mysql:8.3

COPY replica.sh /scripts/replica.sh

RUN chmod u+x /scripts/*.sh

CMD ["sh", "/scripts/replica.sh"]

ENTRYPOINT ["docker-entrypoint.sh"]