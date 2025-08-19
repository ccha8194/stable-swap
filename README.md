**The StableSwap Invariant equation creates a mechanism for trading assets of similar values with minimal slippage:**

<img width="654" height="145" alt="image" src="https://github.com/user-attachments/assets/efc3e79f-7977-44a4-b7b9-9f623b45ce30" />

**A**: Amplification Coefficient which controls how the curve behaves from constant-sum (flat 1:1) to constant-product (Uniswap style). 

**D**: Invariant which stays constant during swaps to ensure trades don't break curve. 

<br>
<br>
<br>

**get_d**

-Returns D invariant given the StableSwapPool current reserve values and amplification coefficient value. To be used as invariant value for the get_dy calculations

-D can't be isolatd in the original equation using basic algebra, which is why we must derive a Newton's Method equation for solving.

<img width="364" height="181" alt="image" src="https://github.com/user-attachments/assets/5bb9df37-a477-470c-bb86-bd7a464e6daf" />

And plugging into Newton's Method Equation:

<img width="231" height="79" alt="image" src="https://github.com/user-attachments/assets/7f057c7d-66eb-44cb-816c-514325a3b87b" />

We simplify into:

<img width="375" height="235" alt="image" src="https://github.com/user-attachments/assets/ad208bf1-d68a-45cf-8ede-316b84dfcb8f" />

With a D Product Term. In the code, we define all relevant terms, then iterate through Newton's Method until returning a converging D value (change in D value is <= 1).

<br>
<br>
<br>
<br>

**get_dy**

get_dy returns the token y that is returned when you input x tokens into the pool. Now, for get_dy(), we essentially have to force one of the token values to take on a new value (Token i) and update the other token (Token j) so that the invariant remains true.
As a result, we define a S' value and a P' value as the sums and products of all balances including the changed balance in Token i and excluding the Token j we are trying to solve for. Note that the fee_bps is applied on the output token

<img width="124" height="78" alt="image" src="https://github.com/user-attachments/assets/a723f65a-2232-420e-b324-cfb298096021" />

We derive f(y) which is 0 when the equation is balanced

<img width="403" height="133" alt="image" src="https://github.com/user-attachments/assets/1ba5d8bd-24d9-475f-8390-d9fd11b20a0c" />

And plug into Newton's Method:

<img width="169" height="61" alt="image" src="https://github.com/user-attachments/assets/e9e35a69-583e-4261-88b1-09f3c928ae63" />

And then simplify into 

<img width="298" height="208" alt="image" src="https://github.com/user-attachments/assets/717b398a-938d-463b-abe2-0ca72a31768a" />

With a C term and B term referenced in code

<img width="163" height="133" alt="image" src="https://github.com/user-attachments/assets/195d0681-f125-43bf-9c08-290f3d353db9" />

Again, we iterate through the Newton's Method until we reach a convering Y value, and then use that value to solve for dy.

<br> 
<br>
<br>
<br>
<br>

**calculate_slippage_bps**

Calculating slippage is fairly straightforward, as the equation is just:

<img width="428" height="70" alt="image" src="https://github.com/user-attachments/assets/95f67d73-867b-4885-87c5-75af17664c51" />

Expected value should be dx, as in a stable swap situation, a user expects what they input into the pool. Due to the nature of the invariant, the actual value varies slightly. 

<br> 
<br>
<br>
<br>
<br>
<br>

**TEST OUTPUTS (Cargo run)**

<img width="565" height="679" alt="image" src="https://github.com/user-attachments/assets/3689a5dd-7675-4e32-97ea-91cecc8577d9" />

Slippage BPS was minimal. Comparisons with the same transaction in a constant product context are also shown. 



