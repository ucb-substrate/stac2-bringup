import argparse
import struct
import serial

BEBE_NOCK_REQ = b'A'
BEBE_NOCK_MAGIC = b"GOBEARS!"

BEBE_CMD_READV  = b'R'
BEBE_CMD_WRITEV = b'W'
BEBE_CMD_JUMP   = b'J'

BEBE_CMD_ACK = b'Y'
BEBE_CMD_NACK = b'N'

parser = argparse.ArgumentParser("bebe_host")
parser.add_argument("--quiet", help="Disable debugging print statements; will only print read output", action='store_true')
parser.add_argument("--wait", help="Wait for DUT to wake up", action='store_true')
parser.add_argument("--port", help="COM port to interact with")
parser.add_argument("--baudrate", help="Baud rate")
parser.add_argument("--addr", help="Address to interact with")
parser.add_argument("--wfile", help="File to write to the DUT")
parser.add_argument("--wdata", help="Write some given data to the DUT")
parser.add_argument("--wlen", help="The length of the data to write to the DUT")
parser.add_argument("--rlen", help="Read length from the DUT")
parser.add_argument("--jump", help="Begin executing at the given address (DUT fence.i's)", action='store_true')
args = parser.parse_args()

ser = serial.Serial(args.port, baudrate=args.baudrate, timeout=1)

def log(*pargs, **kwargs):
    if not args.quiet:
        print(*pargs, **kwargs)

def hexdump(bytes_input, width=16):
    current = 0
    end = len(bytes_input)
    result = ""

    while current < end:
        byte_slice = bytes_input[current : current + width]

        # hex section
        for b in byte_slice:
            result += "%02X " % b

        # filler
        for _ in range(width - len(byte_slice)):
            result += " " * 3
        result += " " * 2

        # printable character section
        for b in byte_slice:
            if (b >= 32) and (b < 127):
                result += chr(b)
            else:
                result += "."

        result += "\n"
        current += width

    return result

def tx(arr:bytes):
    log(hexdump(arr))
    ser.write(arr)
    ser.flush()

def rx(size):
    return ser.read(size)

performed_operation = False
if args.wait:
    performed_operation = True
    log("[bebe host] Waiting for DUT...")
    while True:
        b = rx(1)
        if b == b'A':
            break

    log("[bebe host] DUT found!")

log("[bebe host] Trying to nock...")
tx(BEBE_NOCK_MAGIC)
while True:
    b = rx(1)
    if b == b'A':
        # We might continue to see 'A's for a while after sending the nock due
        # to the delay for handling and FIFOs. Allow this.
        continue
    elif b == BEBE_CMD_ACK:
        break
    else:
        if not args.wait:
            log("[bebe host] Unexpected response from DUT during nock:", str(b))
            exit(1)

log("[bebe host] Connected to DUT!")

if not args.addr:
    if not performed_operation:
        parser.print_help()
        exit(1)
    else:
        exit(0)

addr = int(args.addr, base=16)
if (args.wfile):
    performed_operation = True
    with open(args.wfile, "rb") as fp:
        while True:
            block = fp.read(0xfffff)
            block_len = len(block)
            if block_len == 0:
                break

            header = struct.pack(">IQ", block_len, addr)
            log(f"[bebe host] write {hex(addr)}, len {str(block_len)}...")
            tx(BEBE_CMD_WRITEV + header + block)
            b = rx(1)
            if b != BEBE_CMD_ACK:
                log("[bebe host] Unexpected response from DUT during nock:", str(b))
                exit(1)
            addr += block_len
        log("[bebe host] OK")

if (args.wdata and args.wlen):
    performed_operation = True
    wdata = int(args.wdata, base=16)
    wlen = int(args.wlen)
    if (wlen > 8):
        log(f"[bebe host] wlen {str(wlen)} > 8, which is not allowed. Use wfile.")
    wdata_clip = struct.pack(">Q", wdata)[-wlen:]
    header = struct.pack(">IQ", wlen, addr)
    log(f"[bebe host] write {hex(addr)}={hex(wdata)}, len {str(wlen)}...")
    tx(BEBE_CMD_WRITEV + header + wdata_clip)
    b = rx(1)
    if b != BEBE_CMD_ACK:
        log("[bebe host] Unexpected response from DUT during nock:", str(b))
        exit(1)
    log("[bebe host] OK")

if (args.rlen):
    read_len = int(args.rlen)
    performed_operation = True
    header = struct.pack(">IQ", read_len, addr)
    log(f"[bebe host] read {hex(addr)}, len {str(read_len)}...")
    tx(BEBE_CMD_READV + header)
    log(f"[bebe host] read result:")
    data = rx(read_len)
    if args.quiet:
        print(int.from_bytes(data, "little"))
    else:
        print(hexdump(data))

if (args.jump):
    performed_operation = True
    header = struct.pack(">Q", addr)
    log(f"[bebe host] Jump {hex(addr)}...")
    tx(BEBE_CMD_JUMP + header)
    ack = rx(1)
    if ack != BEBE_CMD_ACK:
        log("[bebe host] Expected ack, got", ack)
    else:
        log("[bebe host] OK")

if not performed_operation:
    parser.print_help()
