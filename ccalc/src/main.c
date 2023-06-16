#include <stdio.h>

#include "localmath.h"
#include "dynomath.h"

int main(int argc, char **argv)
{
    DynoClient_t *c = DYNO_new_client();
    Response_t resp = DYNO_request(c, "https://httpbin.org/json");
    if (resp.error != DYNO_ERROR_OK)
    {
        printf("Error: %d\n", resp.error);
        return resp.error;
    }
    else
    {
        printf("Status: %d", resp.status_code);
        printf("\n%s\n", resp.text);
    }

    DYNO_free_response(resp);
    DYNO_free_client(c);
    return 0;
}