#include <windows.h>
#include <stdio.h>
#include <stdlib.h>

void lockScreen()
{
    LockWorkStation();
}

const char *executeCommand(const char *cmd)
{
    FILE *fp;
    static char total_response[18384] = {0};
    char container[1024] = {0};

    fp = _popen(cmd, "r");
    
    while(fgets(container, 1024, fp) != NULL) 
    {
        strcat(total_response, container);
    }

    _pclose(fp);

    return total_response;
}