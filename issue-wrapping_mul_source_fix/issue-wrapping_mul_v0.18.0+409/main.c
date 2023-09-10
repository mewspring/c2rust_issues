#include <stdio.h>

#include "rnd.h"

int main() {
	set_rand_seed(1234);
	printf("get_rand_seed: 0x%08X\n", (uint32_t)(get_rand_seed()));
	return 0;
}
