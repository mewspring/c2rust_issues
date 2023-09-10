#include <stdint.h>
#include <stdlib.h>

int32_t cur_rand_seed = 0;

void set_rand_seed(int32_t s) {
	cur_rand_seed = s;
}

int32_t get_rand_seed() {
	const uint32_t INCREMENT = 1;
	const uint32_t MULTIPLIER = 0x015A4E35;
	cur_rand_seed = (int32_t)((uint32_t)(MULTIPLIER * ((uint32_t)cur_rand_seed) + INCREMENT));
	int32_t ret = abs(cur_rand_seed);
	return ret;
}
