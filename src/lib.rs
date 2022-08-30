
/**
 * In mathematics, parity is the property of an integer of whether
 * it is even or odd. An integer is even if it is a multiple of two, and odd if it is not.
 
 * Even and odd numbers have opposite parities, e.g., 22 (even number) and 13 (odd number)
 * have opposite parities. In particular, the parity of zero is even. Any two consecutive
 * integers have opposite parity. A number (i.e., integer) expressed in the decimal numeral
 * system is even or odd according to whether its last digit is even or odd. That is, if the
 * last digit is 1, 3, 5, 7, or 9, then it is odd; otherwise it is even—as the last digit of
 * any even number is 0, 2, 4, 6, or 8. The same idea will work using any even base. In particular,
 * a number expressed in the binary numeral system is odd if its last digit is 1; and it is even
 * if its last digit is 0. In an odd base, the number is even according to the sum of its digits—it
 * is even if and only if the sum of its digits is even.
 
 * The division of two whole numbers does not necessarily result in a whole number. For example, 1
 * divided by 4 equals 1/4, which is neither even nor odd, since the concepts of even and odd apply
 * only to integers. But when the quotient is an integer, it will be even if and only if the dividend
 * has more factors of two than the divisor.
 
 * The ancient Greeks considered 1, the monad, to be neither fully odd nor fully even. Some of this
 * sentiment survived into the 19th century: Friedrich Wilhelm August Fröbel's 1826 The Education of Man i
 * nstructs the teacher to drill students with the claim that 1 is neither even nor odd, to which Fröbel
 * attaches the philosophical afterthought,
 
 *   | It is well to direct the pupil's attention here at once to a great far-reaching law of
 *   | nature and of thought. It is this, that between two relatively different things or ideas
 *   | there stands always a third, in a sort of balance, seeming to unite the two. Thus, there
 *   | is here between odd and even numbers one number (one) which is neither of the two. Similarly,
 *   | in form, the right angle stands between the acute and obtuse angles; and in language, the
 *   | semi-vowels or aspirants between the mutes and vowels. A thoughtful teacher and a pupil taught
 *   | to think for himself can scarcely help noticing this and other important laws.
 
 * The even numbers form an ideal in the ring of integers, but the odd numbers do not—this is clear
 * from the fact that the identity element for addition, zero, is an element of the even numbers only.
 * An integer is even if it is congruent to 0 modulo this ideal, in other words if it is congruent to 0
 * modulo 2, and odd if it is congruent to 1 modulo 2.
 
 * A number that is divisible by 2 and generates a remainder of 0 is called an even number. Examples of
 * even numbers are 2, 4, 6, 8, 10, etc. For example, assume you have ten chocolates. These chocolates
 * may be divided into two groups, each having five chocolates. So, ten is an even number.

 * However, 11 chocolates cannot be grouped in this way, so 11 is not an even number. An odd number is
 * a number that is not divisible by 2. The remainder in the case of an odd number is always “1”. 11 is
 * an odd number.
 */

#[no_mangle]
#[warn(non_snake_case)]
pub fn isEven_isEven(num: i32) -> bool {
    num % 2 == 0
}

