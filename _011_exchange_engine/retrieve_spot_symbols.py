import argparse
import requests

def background_requests(api: str, timeout: int):
    
    response = requests.get(api, timeout=timeout)    
    if not response.status_code in [200, 202]:
        print("Error making requests to binance API")
        return {}
    return {"status":response.status_code, "data": response.json()}

def make_binance_requests(api: str, rt: int):
    
    response = requests.get(api)    
    if not response.status_code in [200, 202]:
        print("Error making requests to binance API")
        return 

    data = response.json()
    if rt == 1:
        print(f"* Status-code     : {response.status_code}")
        print(f"* Ticker-Data-Len : {len(data)}")
        print()
        while True: 
            print()
            print("0. Exit")
            print("1. Get full list of tickers with prices")
            print("2. Get a single ticker with price")
            
            try:
                opt = int(input("\nPlease select from the above option: "))
                if opt > 2:
                    print("Please select either 0, 1, or 2")
                    continue
                if opt == 0:
                    break
                elif opt == 1:
                    for ind,d in enumerate(data):
                        print(f"{ind+1}. {d["symbol"]} > {d["price"]}")
                elif opt == 2:
                    ticker = input("\nPlease enter a valid symbol: ")
                    price = ""
                    for d in data:
                        if d["symbol"] == ticker:
                            # print(f"{d["symbol"]} > {d["price"]}")
                            price = d["price"]
                            break
                    if price:
                        print(f"\n{ticker} > {price}")
                    else:
                        print(f"\n{ticker} not found!!")
            except ValueError:
                print("That wasn't a valid whole number")
                break
    else:
        print(data.keys()) 

if __name__ == "__main__":

    TICKER_PRICES="https://api.binance.com/api/v3/ticker/price"
    EXCHANGE_INFO="https://api.binance.com/api/v3/exchangeInfo"
    
    parser = argparse.ArgumentParser(description="A CLI tool to fetch spot symbols from BINANCE API")
    parser.add_argument(
        "--exchange-api-type", 
        type=int, 
        default=1, 
        choices=[1, 2],
        help="1 to get list of ticker prices, and 2 to get full exchanges info"
        )
    args = parser.parse_args()
    
    # print(args)
    if args.exchange_api_type == 1:
        print("----------------------------------- FETCH ONLY TICKER PRICES -------------------------------------------")
        make_binance_requests(TICKER_PRICES, 1)
    else:
        print("---------------- FETCH EXCHANGE INFO (list of all symbols, trading rules, and statuses) ----------------")
        make_binance_requests(EXCHANGE_INFO, 2)

    print("\nGood Bye!!!")