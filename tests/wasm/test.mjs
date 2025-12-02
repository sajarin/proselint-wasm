// WASM Integration Tests for Node.js
// Run with: node tests/wasm/test.mjs
// Requires: wasm-pack build --target nodejs

import { readFile } from 'fs/promises';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const pkgDir = join(__dirname, '../../pkg');

let passed = 0;
let failed = 0;

function assert(condition, message) {
  if (condition) {
    passed++;
    console.log(`✅ ${message}`);
  } else {
    failed++;
    console.error(`❌ ${message}`);
  }
}

function assertEquals(actual, expected, message) {
  if (actual === expected) {
    passed++;
    console.log(`✅ ${message}`);
  } else {
    failed++;
    console.error(`❌ ${message}`);
    console.error(`   Expected: ${expected}`);
    console.error(`   Actual: ${actual}`);
  }
}

async function runTests() {
  console.log('🧪 Running WASM Integration Tests\n');

  try {
    // Import WASM module
    const wasm = await import(join(pkgDir, 'proselint_wasm.js'));

    console.log('📦 Basic Functionality Tests\n');

    // Test 1: Create instance
    const proselint = new wasm.Proselint();
    assert(proselint !== null, 'Can create Proselint instance');

    // Test 2: Basic linting
    const results = proselint.lint('This is very very important.');
    const parsed = JSON.parse(results);
    assert(Array.isArray(parsed), 'lint() returns array');
    assert(parsed.length > 0, 'Detects "very very" weasel words');

    // Test 3: Check result structure
    if (parsed.length > 0) {
      const result = parsed[0];
      assert('check' in result, 'Result has check field');
      assert('message' in result, 'Result has message field');
      assert('line' in result, 'Result has line field');
      assert('column' in result, 'Result has column field');
      assert('start' in result, 'Result has start field');
      assert('end' in result, 'Result has end field');
      assert('severity' in result, 'Result has severity field');
    }

    // Test 4: Empty text
    const emptyResults = JSON.parse(proselint.lint(''));
    assertEquals(emptyResults.length, 0, 'Empty text returns no results');

    // Test 5: Clean text
    const cleanResults = JSON.parse(proselint.lint('The cat sat on the mat.'));
    assertEquals(cleanResults.length, 0, 'Clean text returns no results');

    console.log('\n📦 UTF-8 Tests\n');

    // Test 6: UTF-8 with emoji
    const emojiText = 'This is very 👋 important.';
    const emojiResults = JSON.parse(proselint.lint(emojiText));
    assert(emojiResults.length > 0, 'Handles emoji in text');
    assert(emojiResults[0].line >= 1, 'Line number is valid with emoji');
    assert(emojiResults[0].column >= 1, 'Column number is valid with emoji');

    // Test 7: UTF-8 with accented characters
    const accentText = 'Café is very élégant.';
    const accentResults = JSON.parse(proselint.lint(accentText));
    assert(accentResults.length > 0, 'Handles accented characters');

    // Test 8: UTF-8 with CJK
    const cjkText = 'This is very 日本語 text.';
    const cjkResults = JSON.parse(proselint.lint(cjkText));
    assert(cjkResults.length > 0, 'Handles CJK characters');

    console.log('\n📦 Error Handling Tests\n');

    // Test 9: Oversized text
    const largeText = 'x'.repeat(11 * 1024 * 1024); // 11MB
    const largeResult = proselint.lint(largeText);
    const largeParsed = JSON.parse(largeResult);
    assert('error' in largeParsed, 'Returns error for oversized text');
    assert(largeParsed.error.includes('too large'), 'Error message mentions size limit');

    // Test 10: Batch linting
    const batchInput = JSON.stringify(['This is very bad.', 'Really quite good.']);
    const batchResult = proselint.lint_batch(batchInput);
    const batchParsed = JSON.parse(batchResult);
    assert(Array.isArray(batchParsed), 'Batch returns array');
    assertEquals(batchParsed.length, 2, 'Batch processes all texts');
    assert(Array.isArray(batchParsed[0]), 'Each batch result is array');

    // Test 11: Batch oversized
    const oversizedBatch = JSON.stringify(new Array(101).fill('test'));
    const oversizedResult = proselint.lint_batch(oversizedBatch);
    const oversizedParsed = JSON.parse(oversizedResult);
    assert('error' in oversizedParsed, 'Returns error for oversized batch');
    assert(oversizedParsed.error.includes('too large'), 'Error mentions batch size');

    // Test 12: Invalid batch JSON
    const invalidBatch = proselint.lint_batch('not json');
    const invalidParsed = JSON.parse(invalidBatch);
    assert('error' in invalidParsed, 'Returns error for invalid JSON');

    console.log('\n📦 Configuration Tests\n');

    // Test 13: Custom config
    const config = JSON.stringify({ check_quotes: false, checks: { weasel_words: false } });
    const customLinter = wasm.Proselint.with_config(config);
    const configResults = JSON.parse(customLinter.lint('This is "very good" text.'));
    // Should not detect "very" because check_quotes: false and weasel_words disabled
    const hasWeaselWords = configResults.some(r => r.check.includes('weasel'));
    assert(!hasWeaselWords, 'Config disables weasel word checks');

    console.log('\n📦 Utility Method Tests\n');

    // Test 14: lint_count
    const count = proselint.lint_count('This is very very bad.');
    assert(count > 0, 'lint_count returns number > 0');
    assert(typeof count === 'number', 'lint_count returns number');

    // Test 15: version
    const version = wasm.Proselint.version();
    assert(typeof version === 'string', 'version() returns string');
    assert(version.length > 0, 'version() returns non-empty string');

    // Test 16: available_checks
    const checks = wasm.Proselint.available_checks();
    const checksParsed = JSON.parse(checks);
    assert(Array.isArray(checksParsed), 'available_checks() returns array');
    assert(checksParsed.length > 0, 'Has available checks');

    // Test 17: warm_all
    const warmCount = proselint.warm_all();
    assert(typeof warmCount === 'number', 'warm_all() returns number');
    assert(warmCount > 0, 'warm_all() pre-compiles checks');

    console.log('\n📦 Line Ending Tests\n');

    // Test 18: Unix line endings
    const unixText = 'Line 1\nLine 2\nThis is very bad.';
    const unixResults = JSON.parse(proselint.lint(unixText));
    assert(unixResults.length > 0, 'Handles Unix line endings');
    assert(unixResults[0].line === 3, 'Correct line number with \\n');

    // Test 19: Windows line endings
    const winText = 'Line 1\r\nLine 2\r\nThis is very bad.';
    const winResults = JSON.parse(proselint.lint(winText));
    assert(winResults.length > 0, 'Handles Windows line endings');
    assert(winResults[0].line === 3, 'Correct line number with \\r\\n');

    // Test 20: Mixed line endings
    const mixedText = 'Line 1\nLine 2\r\nThis is very bad.';
    const mixedResults = JSON.parse(proselint.lint(mixedText));
    assert(mixedResults.length > 0, 'Handles mixed line endings');

    console.log('\n' + '='.repeat(50));
    console.log(`✅ Passed: ${passed}`);
    console.log(`❌ Failed: ${failed}`);
    console.log('='.repeat(50));

    process.exit(failed > 0 ? 1 : 0);

  } catch (error) {
    console.error('💥 Fatal error:', error);
    process.exit(1);
  }
}

runTests();
