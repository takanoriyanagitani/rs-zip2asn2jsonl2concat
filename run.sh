#!/bin/sh

input=./sample.d/input.zip

geninput1(){
	printf '\x04' > ./sample.d/jsonl.fragment.tag.asn1.dat
	jq -c -n '{name:"fuji",height:3.776}' |
		gzip --fast > ./sample.d/jsonl.fragment.val.asn1.dat
	len=$( cat ./sample.d/jsonl.fragment.val.asn1.dat | wc -c )
	printf '%x' $len | xxd -r -ps > ./sample.d/jsonl.fragment.len.asn1.dat
	cat ./sample.d/jsonl.fragment.{tag,len,val}.asn1.dat \
		> ./sample.d/jsonl.asn1.dat

	printf '\x04' > ./sample.d/names.fragment.tag.asn1.dat
    printf 'name1.jsonl\n' |
		gzip --fast > ./sample.d/names.fragment.val.asn1.dat
	len=$( cat ./sample.d/names.fragment.val.asn1.dat | wc -c )
	printf '%x' $len | xxd -r -ps > ./sample.d/names.fragment.len.asn1.dat
	cat ./sample.d/names.fragment.{tag,len,val}.asn1.dat \
		> ./sample.d/names.asn1.dat

	printf '\x30' > ./sample.d/items.fragment.tag.asn1.dat
	cat \
		./sample.d/names.asn1.dat \
		./sample.d/jsonl.asn1.dat \
		> ./sample.d/items.fragment.val.asn1.dat
	len=$( cat ./sample.d/items.fragment.val.asn1.dat | wc -c )
	printf '%x' $len | xxd -r -ps > ./sample.d/items.fragment.len.asn1.dat
	cat ./sample.d/items.fragment.{tag,len,val}.asn1.dat \
		> ./sample.d/items1.asn1.dat
}

geninput2(){
	printf '\x04' > ./sample.d/jsonl.fragment.tag.asn1.dat
	jq -c -n '{name:"takao",height:0.599}' |
		gzip --fast > ./sample.d/jsonl.fragment.val.asn1.dat
	len=$( cat ./sample.d/jsonl.fragment.val.asn1.dat | wc -c )
	printf '%x' $len | xxd -r -ps > ./sample.d/jsonl.fragment.len.asn1.dat
	cat ./sample.d/jsonl.fragment.{tag,len,val}.asn1.dat \
		> ./sample.d/jsonl.asn1.dat

	printf '\x04' > ./sample.d/names.fragment.tag.asn1.dat
    printf 'name1.jsonl\n' |
		gzip --fast > ./sample.d/names.fragment.val.asn1.dat
	len=$( cat ./sample.d/names.fragment.val.asn1.dat | wc -c )
	printf '%x' $len | xxd -r -ps > ./sample.d/names.fragment.len.asn1.dat
	cat ./sample.d/names.fragment.{tag,len,val}.asn1.dat \
		> ./sample.d/names.asn1.dat

	printf '\x30' > ./sample.d/items.fragment.tag.asn1.dat
	cat \
		./sample.d/names.asn1.dat \
		./sample.d/jsonl.asn1.dat \
		> ./sample.d/items.fragment.val.asn1.dat
	len=$( cat ./sample.d/items.fragment.val.asn1.dat | wc -c )
	printf '%x' $len | xxd -r -ps > ./sample.d/items.fragment.len.asn1.dat
	cat ./sample.d/items.fragment.{tag,len,val}.asn1.dat \
		> ./sample.d/items2.asn1.dat
}

geninput(){
	echo generating input...
	mkdir -p sample.d

	geninput1
	geninput2

	ls ./sample.d/items[12].asn1.dat |
		zip \
			-0 \
			-@ \
			-T \
			-v \
			-o \
			"${input}"
}

test -f "${input}" || geninput

wazero \
	run \
	-env ENV_INPUT_ZIP_FILENAME=/guest-i.d/input.zip \
	-mount "${PWD}/sample.d:/guest-i.d:ro" \
	./rs-zip2asn2jsonl2concat.wasm
