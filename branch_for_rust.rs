fn main()
{
	// while
	let mut n = 0;	// ループ変数は書き換えるのでmutable
	while n<3
	{
		println!("n:{}",n);
		n +=1;				// n++とは書けないので注意
	}

	println!("");

	// for
	// 範囲演算子で繰返し回数を指定
	for i in 2..5
	{
		println!("i:{}",i);
	}
	println!("");

	// 配列の和だけ繰り返し
	let a=["one", "two","three"];
	for j in a
	{
		println!("a:{}",j);		// jには繰返し回数ではなく配列要素が入る
	}

}
