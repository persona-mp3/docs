package jchat;
import java.net.Socket;
import java.io.BufferedReader;
import java.io.PrintWriter;
import java.io.InputStreamReader;
import java.util.concurrent.ConcurrentLinkedQueue;

class Client {
	public static void main(String[] args) throws Exception{
		String ip = "localhost";
		int port = 1738;

		if (args.length < 2) {
			String warn =String.format("warning:: Using default values:: ip=%s port=%d", ip, port);
			println(warn);
		}

		String line = "";
		ConcurrentLinkedQueue<String> channel = new ConcurrentLinkedQueue<String>();
		String latestMsg = "";
		try (
				Socket conn  = new Socket(ip, port);
				BufferedReader reader = new BufferedReader(new InputStreamReader(conn.getInputStream()));
				PrintWriter writer = new PrintWriter(conn.getOutputStream(), true);
				){
					writer.println("hey bruh, it's me..");
					ReadFromStdin stdinRunner = new ReadFromStdin(channel, writer);
					Thread writerThread= new Thread(stdinRunner);
					writerThread.start();
					while (conn.isConnected() && !conn.isClosed()) {
						if (line == null ) {
							println("server has stopped sending, possibly disconnected, EOF");
							break;
						}
						 
						// latestMsg = channel.poll();
						// writer.println(latestMsg);


						line = reader.readLine();
						println("response:: " + line);
					}
				}

	}

	static class ReadFromStdin implements Runnable {
		// 1. Need a channel to communicate with server
		// 2. Need to read from stdin
		// 3. Might want to change channel to be ArrayBlockingQueue, later on
		ConcurrentLinkedQueue channel;
		PrintWriter writer;
		public ReadFromStdin(ConcurrentLinkedQueue channel, PrintWriter writer){
			this.channel = channel;
			this.writer = writer;
		}
		static String msg = "";
		boolean success = false;

		public void main() throws Exception {
			try( BufferedReader stdin = new BufferedReader(new InputStreamReader(System.in));)
			{
				println("reading from stdin....");
				// In the actual impl, might want to use signals instead
				while (( msg = stdin.readLine()) != null ){
				println("recvdMsg:: " + msg);
				success = channel.offer(msg);
				if (!success) {
					eprintln("could not offer:: " + msg + " to queue/channel");
				}

				writer.println(msg);
				}
			}
		}

		@Override 
		public void run(){
			println("stdin thread running");
			try {
				main();
			} catch(Exception err) {
				eprintln("writerThreadErr");
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
