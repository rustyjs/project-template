import { hello } from '..';

describe('Tests', () => {
	test('should pass', () => {
		expectTypeOf(hello()).toBeString();
		expect(hello()).toBe('world');
	});
});
