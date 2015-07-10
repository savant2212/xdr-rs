#include <stdio.h>
#include <string.h>    //strlen
#include <sys/socket.h>
#include <arpa/inet.h> //inet_addr
#include <unistd.h>    //write
#include <stdint.h>
#include <rpc/xdr.h>

#define check(name, mod) {if(d_ ## name != r_ ## name) {printf("%s failed(%" mod  " != %"  mod  ")\n",#name, d_##name , r_##name);}}

int main(int argc , char *argv[])
{
	int8_t d_i8, r_i8;
	uint8_t d_u8, r_u8;
	int16_t d_i16, r_i16;
	uint16_t d_u16, r_u16;
	int32_t d_i32, r_i32;
	uint32_t d_u32,r_u32;
	float d_f32, r_f32;
	double d_f64, r_f64;

	XDR xdr;
	int sock;
	struct sockaddr_in server;
	char message[36] , server_reply[36];

	d_i8 = -1;
	d_u8 = 1;
	d_i16 = -256;
	d_u16 = 512;
	d_i32 = -0x0FFFAAAA;
	d_u32 = 0xDEADBEEF;
	d_f32 = 8.800f;
	d_f64 = 6.66;

	xdrmem_create(&xdr, message, 36, XDR_ENCODE);

	xdr_u_char(&xdr, &d_u8);
	xdr_char(&xdr, &d_i8);
	xdr_u_short(&xdr, &d_u16);
	xdr_short(&xdr, &d_i16);
	xdr_u_int(&xdr, &d_u32);
	xdr_int(&xdr, &d_i32);
	xdr_float(&xdr, &d_f32);
	xdr_double(&xdr, &d_f64);

	xdr_destroy(&xdr);

	//Create socket
	sock = socket(AF_INET , SOCK_STREAM , 0);
	if (sock == -1)
	{
		printf("Could not create socket");
	}
	puts("Socket created");

	server.sin_addr.s_addr = inet_addr("127.0.0.1");
	server.sin_family = AF_INET;
	server.sin_port = htons( 9123 );

	//Connect to remote server
	if (connect(sock , (struct sockaddr *)&server , sizeof(server)) < 0)
	{
		perror("connect failed. Error");
		return 1;
	}
	puts("Connected\n");
	//keep communicating with server
	while(1)
	{
		//Send some data
		if( send(sock , message , 36 , 0) < 0)
		{
			puts("Send failed");
			return 1;
		}
		//Receive a reply from the server
		if( recv(sock , server_reply , 36 , 0) < 0)
		{
			puts("recv failed");
			break;
		}
		xdrmem_create(&xdr, server_reply, 36, XDR_DECODE);

		xdr_u_char(&xdr, &r_u8);
		xdr_char(&xdr, &r_i8);
		xdr_u_short(&xdr, &r_u16);
		xdr_short(&xdr, &r_i16);
		xdr_u_int(&xdr, &r_u32);
		xdr_int(&xdr, &r_i32);
		xdr_float(&xdr, &r_f32);
		xdr_double(&xdr, &r_f64);

		check(i8, "i");
		check(u8, "u");
		check(i16, "i");
		check(u16, "u");
		check(i32, "i");
		check(u32, "u");
		check(f32, "f");
		check(f64, "f");

		break;
	}

	close(sock);
	return 0;
}
