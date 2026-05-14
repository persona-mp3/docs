package jchat;
import java.net.Socket;
import java.net.ServerSocket;
import java.io.BufferedReader;
import java.io.PrintWriter;
import java.io.InputStreamReader;

class Server {
	public static void main(String[] args) throws Exception {
		int port = 1738;
		println("hello from server");
		if (args.length < 1) {
			println("port not provided, using default port::" + String.valueOf(port));
		}

		try(
				ServerSocket listener = new ServerSocket(port)
			 ){
			println("server running:: tcp://localhost:" +  String.valueOf(port));

			while (!listener.isClosed()) {
				Socket conn = listener.accept();
				println("accepted new connetion from " + conn.getRemoteSocketAddress());

				HandleClient hc = new HandleClient(conn);
				Thread clientThread = new Thread(hc);
				clientThread.start();
			}
		} catch(Exception err) {
			eprintln("An application error occured:: " + err.getMessage());
			err.printStackTrace();
		}
	}

	static String fmtString = "";

	static class HandleClient implements Runnable {
		Socket conn;
		public HandleClient(Socket conn){
			this.conn = conn;
		}

		public void main() throws Exception {
			String line = "";
			try (
					BufferedReader reader = new BufferedReader(new InputStreamReader(this.conn.getInputStream()));
					PrintWriter writer = new PrintWriter(this.conn.getOutputStream(), true);
					){
				println("reading from client...");
				while (this.conn.isConnected() && !this.conn.isClosed()) {
					line = reader.readLine();
					if (line == null ) {
						println("client has stopped sending, possibly disconnected, EOF");
						break;
					}
					println("request:: " + line);

					writer.println("wassup bruh");

				}
					}
		}

		@Override
		public void run(){
			println("Handle Client Thread impls runnable");
			try {
				main();
			} catch(Exception err) {
				eprintln("error handling client");
				eprintln(err);
			}
		}
	}

	static void println(Object s) {
		System.out.println(s);
	}

	static void eprintln(Object s) {
		System.err.println(s);
	}

}
