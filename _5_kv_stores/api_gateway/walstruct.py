import os
import zlib

class WALKVStore:
    def __init__(self, path: str):
        
        self.logs_folder = path + "/wallogs"
        if not os.path.isdir(self.logs_folder):
            print("Logs folder doesn't exists. Creating")
            os.makedirs(self.logs_folder)

        all_logs = os.listdir(self.logs_folder)
        
        self.wal_filename = "wal-00001.log" if len(all_logs) == 0 else all_logs[-1]
        
        self.wal_file_path = self.logs_folder + "/" + self.wal_filename
        
        self.wal_file = open(self.wal_file_path, "ab+")
        self.wal_file.seek(0, os.SEEK_END)
        
    def change_to_new_walfile(self, file_size):
        f_num = self.wal_filename[4:-4]
        f_int_num = str(int(f_num) + 1)
        
        self.wal_filename = "wal-" + "0"*(5-len(f_int_num)) + f_int_num + ".log"
        self.wal_file_path = self.logs_folder + "/" + self.wal_filename
        
        self.wal_file = open(self.wal_file_path, "ab+")
        self.wal_file.seek(0, os.SEEK_END)
        
    def write_record(self, kvs: dict):
        
        file_size = os.path.getsize(self.wal_file_path)
        if file_size > 67108864: # Create a new file once the log file size exceeds 64MB (67108864)
            self.change_to_new_walfile(file_size)    
        
        # Create a payload
        payload = f"key:{kvs['key']};value:{kvs['value']};timestamp:{kvs['timestamp']}"
        length = len(payload)
        
        # CRC calculation
        crc = zlib.crc32(payload.encode("utf-8")) & 0xffffffff
        
        # Get LSN as current byte offset
        lsn = self.wal_file.tell()
        
        # Create payload
        payload = f"lsn:{lsn};crc:{crc};length:{length};{payload}\n"
        
        # Write WAL entry
        self.wal_file.write(payload.encode("utf-8"))
        self.wal_file.flush()
        os.fsync(self.wal_file.fileno())