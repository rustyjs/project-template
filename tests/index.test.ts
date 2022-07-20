import { hello } from '..';

describe('Tests', () => {
	test('should pass', () => {
		expect(hello()).toBe('world');
	});
});
