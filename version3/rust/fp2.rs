/*
Licensed to the Apache Software Foundation (ASF) under one
or more contributor license agreements.  See the NOTICE file
distributed with this work for additional information
regarding copyright ownership.  The ASF licenses this file
to you under the Apache License, Version 2.0 (the
"License"); you may not use this file except in compliance
with the License.  You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing,
software distributed under the License is distributed on an
"AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
KIND, either express or implied.  See the License for the
specific language governing permissions and limitations
under the License.
*/

use xxx::fp::FP;
use xxx::big::BIG;
use xxx::dbig::DBIG;
use xxx::rom;

#[derive(Copy, Clone)]
pub struct FP2 {
	a:FP,
	b:FP,
}

impl FP2 {

	pub fn new() -> FP2 {
		FP2 {
				a: FP::new(),
				b: FP::new(),
		}
	}

	pub fn new_int(a: isize) -> FP2 {
		let mut f=FP2::new();
		f.a.copy(&FP::new_int(a));
		f.b.zero();
		return f;
	}	

	pub fn new_copy(x: &FP2) -> FP2 {
		let mut f=FP2::new();
		f.a.copy(&x.a);
		f.b.copy(&x.b);
		return f
	}	

	pub fn new_fps(c: &FP,d: &FP) -> FP2 {
		let mut f=FP2::new();
		f.a.copy(c);
		f.b.copy(d);
		return f;
	}	

	pub fn new_bigs(c: &BIG,d: &BIG) -> FP2 {
		let mut f=FP2::new();
		f.a.copy(&FP::new_big(c));
		f.b.copy(&FP::new_big(d));
		return f;
	}	

	pub fn new_fp(c: &FP) -> FP2 {
		let mut f=FP2::new();
		f.a.copy(c);
		f.b.zero();
		return f;
	}	

	pub fn new_big(c: &BIG) -> FP2 {
		let mut f=FP2::new();
		f.a.copy(&FP::new_big(c));
		f.b.zero();
		return f;
	}	

/* reduce components mod Modulus */
	pub fn reduce(&mut self) {
		self.a.reduce();
		self.b.reduce();
	}

/* normalise components of w */
	pub fn norm(&mut self) {
		self.a.norm();
		self.b.norm();
	}	

/* test self=0 ? */
	pub fn iszilch(&mut self) -> bool {
		self.reduce();
		return self.a.iszilch() && self.b.iszilch();
	}	

	pub fn cmove(&mut self,g:&FP2,d: isize) {
		self.a.cmove(&g.a,d);
		self.b.cmove(&g.b,d);
	}		

/* test self=1 ? */
	pub fn isunity(&mut self) -> bool {
		let mut one=FP::new_int(1);
		return self.a.equals(&mut one) && self.b.iszilch();
	}

/* test self=x */
	pub fn equals(&mut self,x:&mut FP2) -> bool {
		return self.a.equals(&mut x.a) && self.b.equals(&mut x.b);
	}

/* extract a */
	pub fn geta(&mut self) -> BIG { 
		return self.a.redc();
	}

/* extract b */
	pub fn getb(&mut self) -> BIG {
		return self.b.redc();
	}

/* copy self=x */
	pub fn copy(&mut self,x :&FP2) {
		self.a.copy(&x.a);
		self.b.copy(&x.b);
	}

/* set self=0 */
	pub fn zero(&mut self) {
		self.a.zero();
		self.b.zero();
	}

/* set self=1 */
	pub fn one(&mut self) {
		self.a.one();
		self.b.zero();
	}	

/* negate self mod Modulus */
	pub fn neg(&mut self) {
	//	self.norm();
		let mut m=FP::new_copy(&self.a);
		let mut t=FP::new();

		m.add(&self.b);
		m.neg();
	//	m.norm();
		t.copy(&m); t.add(&self.b);
		self.b.copy(&m);
		self.b.add(&self.a);
		self.a.copy(&t);
	}	

/* set to a-ib */
	pub fn conj(&mut self) {
		self.b.neg();
		self.b.norm();
	}

/* self+=a */
	pub fn add(&mut self,x:&FP2) {
		self.a.add(&x.a);
		self.b.add(&x.b);
	}

	pub fn dbl(&mut self) {
		self.a.dbl();
		self.b.dbl();
	}

/* self-=a */
	pub fn sub(&mut self,x:&FP2) {
		let mut m=FP2::new_copy(x);
		m.neg();
		self.add(&m);
	}

/* self*=s, where s is an FP */
	pub fn pmul(&mut self,s:&FP) {
		self.a.mul(s);
		self.b.mul(s);
	}

/* self*=i, where i is an int */
	pub fn imul(&mut self,c: isize) {
		self.a.imul(c);
		self.b.imul(c);
	}

/* self*=self */
	pub fn sqr(&mut self) {
		let mut w1=FP::new_copy(&self.a);
		let mut w3=FP::new_copy(&self.a);
		let mut mb=FP::new_copy(&self.b);

	//	w3.mul(&self.b);
		w1.add(&self.b);

w3.add(&self.a);
w3.norm();
self.b.mul(&w3);

		mb.neg();
		self.a.add(&mb);

		w1.norm();
		self.a.norm();

		self.a.mul(&w1);
	//	self.b.copy(&w3); self.b.add(&w3);

	//	self.b.norm();
	}	

/* this*=y */
	pub fn mul(&mut self,y :&FP2) {

		let exa=FP::excess(&self.a.x);
		let exb=FP::excess(&self.b.x);
		let eya=FP::excess(&y.a.x);
		let eyb=FP::excess(&y.b.x);
		let ec=exa+exb+1;
		let ed=eya+eyb+1;

		if FP::pexceed(ec,ed) {
			if exa>0 {self.a.reduce()}
			if exb>0 {self.b.reduce()}
		}


//		if FP::pexceed(FP::excess(&(self.a.x))+FP::excess(&(self.b.x))+1,FP::excess(&(y.a.x))+FP::excess(&(y.b.x))+1) {
//			self.reduce();
//		}

		//if FP::pexceed(FP::excess(&self.a.x),FP::excess(&y.a.x)) {
		//	self.a.reduce()
		//}
		//if FP::pexceed(FP::excess(&self.b.x),FP::excess(&y.b.x)) {
		//	self.b.reduce()
		//}


  		let p = BIG::new_ints(&rom::MODULUS);    
  		let mut pr=DBIG::new();

  		pr.ucopy(&p);

		let mut c=BIG::new_copy(&(self.a.x));
		let mut d=BIG::new_copy(&(y.a.x));

		let mut a=BIG::mul(&self.a.x,&y.a.x);
		let mut b=BIG::mul(&self.b.x,&y.b.x);

		c.add(&self.b.x); c.norm();
		d.add(&y.b.x); d.norm();

		let mut e=BIG::mul(&c,&d);
		let mut f=DBIG::new_copy(&a); f.add(&b);
		b.rsub(&pr);

		a.add(&b); a.norm();
		e.sub(&f); e.norm();

		self.a.x.copy(&FP::modulo(&mut a));
		self.b.x.copy(&FP::modulo(&mut e));

/*
		let mut w1=FP::new_copy(&self.a);
		let mut w2=FP::new_copy(&self.b);
		let mut w5=FP::new_copy(&self.a);
		let mut mw=FP::new();

		w1.mul(&my.a);  // w1=a*y.a  
		w2.mul(&my.b);  // w2=b*y.b 
		w5.add(&self.b);    // w5=a+b
		self.b.copy(&my.a); self.b.add(&my.b); // b=y.a+y.b

		self.b.norm();
		w5.norm();

		self.b.mul(&w5);
		mw.copy(&w1); mw.add(&w2); mw.neg();

		self.b.add(&mw); mw.add(&w1);
		self.a.copy(&w1); self.a.add(&mw);

		self.norm(); */
	}

/* sqrt(a+ib) = sqrt(a+sqrt(a*a-n*b*b)/2)+ib/(2*sqrt(a+sqrt(a*a-n*b*b)/2)) */
/* returns true if this is QR */
	pub fn sqrt(&mut self) -> bool {
		if self.iszilch() {return true}
		let mut w1=FP::new_copy(&self.b);
		let mut w2=FP::new_copy(&self.a);
		w1.sqr(); w2.sqr(); w1.add(&w2);
		if w1.jacobi()!=1 { self.zero(); return false }
		w2.copy(&w1.sqrt()); w1.copy(&w2);
		w2.copy(&self.a); w2.add(&w1); w2.norm(); w2.div2();
		if w2.jacobi()!=1 {
			w2.copy(&self.a); w2.sub(&w1); w2.norm(); w2.div2();
			if w2.jacobi()!=1 { self.zero(); return false }
		}
		w1.copy(&w2.sqrt());
		self.a.copy(&w1);
		w1.dbl();
		w1.inverse();
		self.b.mul(&w1);
		return true;
	}

/* output to hex string */
	pub fn tostring(&mut self) -> String {
		return format!("[{},{}]",self.a.tostring(),self.b.tostring());		
	}

/* self=1/self */
	pub fn inverse(&mut self) {
		self.norm();
		let mut w1=FP::new_copy(&self.a);
		let mut w2=FP::new_copy(&self.b);

		w1.sqr();
		w2.sqr();
		w1.add(&w2);
		w1.inverse();
		self.a.mul(&w1);
		w1.neg(); w1.norm();
		self.b.mul(&w1);
	}

/* self/=2 */
	pub fn div2(&mut self) {
		self.a.div2();
		self.b.div2();
	}

/* self*=sqrt(-1) */
	pub fn times_i(&mut self) {
	//	a.norm();
		let z=FP::new_copy(&self.a);
		self.a.copy(&self.b); self.a.neg();
		self.b.copy(&z);
	}

/* w*=(1+sqrt(-1)) */
/* where X*2-(1+sqrt(-1)) is irreducible for FP4, assumes p=3 mod 8 */
	pub fn mul_ip(&mut self) {
	//	self.norm();
		let t=FP2::new_copy(self);
		let z=FP::new_copy(&self.a);
		self.a.copy(&self.b);
		self.a.neg();
		self.b.copy(&z);
		self.add(&t);
	//	self.norm();
	}

/* w/=(1+sqrt(-1)) */
	pub fn div_ip(&mut self) {
		let mut t=FP2::new();
		self.norm();
		t.a.copy(&self.a); t.a.add(&self.b);
		t.b.copy(&self.b); t.b.sub(&self.a);
		t.norm(); self.copy(&t); 
		self.div2();
	}

}
/*
fn main()
{
	let mut x=FP2::new();
}
*/
