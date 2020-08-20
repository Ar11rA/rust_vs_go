package problems

type Cashier struct {
	threshold int
	ordersProcessed int
	discount int
	products []int
	prices []int
}


func Constructor(n int, discount int, products []int, prices []int) Cashier {
	return Cashier{
		threshold: n,
		discount: discount,
		products: products,
		prices: prices,
		ordersProcessed: 1,
	}
}


func (c *Cashier) GetBill(product []int, amount []int) float64 {
	if len(product) != len(amount) {
		panic("Invalid args!")
	}
	productIndicesMap := make(map[int]int)
	bill := 0.0
	for index, product := range c.products {
		productIndicesMap[product] = index
	}
	for index, prod := range product {
		productIndex := productIndicesMap[prod]
		bill += float64(c.prices[productIndex] * amount[index])
	}
	if c.ordersProcessed % c.threshold == 0 {
		bill -= (float64(c.discount) * bill) / 100.0
	}
	c.ordersProcessed += 1
	return bill
}