// This file is part of the uutils acl package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use uutests::{new_ucmd, util::TestScenario, util_name};

#[test]
fn test_invalid_arg() {
    new_ucmd!().arg("--definitely-invalid").fails().code_is(1);
}
