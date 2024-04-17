#!bin/sh

OKTA_DOMAIN="https://dev-03131721.okta.com/oauth2/default"
CLIENT_ID="0oagdcnu3rhx5NHu55d7"
OIDC_USERNAME="vinicinbgs@github.oktaidp"

minikube start \
    --extra-config=apiserver.oidc-issuer-url=$OKTA_DOMAIN \
    --extra-config=apiserver.oidc-client-id=$CLIENT_ID \
    --extra-config=apiserver.oidc-username-prefix=oidc: \
    --extra-config=apiserver.oidc-username-claim=sub \
    --extra-config=apiserver.oidc-groups-prefix=oidc: \
    --extra-config=apiserver.oidc-groups-claim=groups