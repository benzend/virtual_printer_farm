from fastapi import FastAPI
from typing import Optional
import uuid
from pydantic import BaseModel
import subprocess, logging
import yaml

app = FastAPI()

class DatabaseService:
    def __init__(self):
        self.source = "./"


    def get_collection_count(self, collection: str):
            
        try:
            with open("{}/{}.yml".format(self.source, collection), "r") as file:
                data = yaml.safe_load(file)
                count = data["{}_count".format(collection)]
        except Exception as e: 
            logging.error(e)
            raise self.Collection404()

        return count


    def get_collection(self, collection: str):
        try:
            with open("{}/{}.yml".format(self.source, collection), "r") as file:
                data = yaml.safe_load(file)
                items = data[collection]
        except Exception as e:
            logging.error(e)
            raise self.Collection404()

        return items
        

    def get_by_id(self, collection: str, id: int):
        col = None
        try:
            col = self.get_collection(collection)
        except self.Collection404 as e:
            logging.error(e.message)
        except Exception as e:
            logging.error(e)

        if not col:
            raise self.Collection404()

        item = next((x for x in col if x['id'] == id), None)


        if not item:
            raise self.Item404()

        return item


    def create_collection(self, collection):
        try:
            with open("{}/{}.yml".format(self.source, collection), "w") as file:
                documents = yaml.dump({collection: [], collection + '_count': 0}, file)
        except Exception as e:
            logging.error(e)
            raise self.CollectionCreateErr()

        return documents


    def add_to_collection(self, collection, payload):
        try:
            col = self.get_collection(collection)
        except self.Collection404 as e:
            raise self.Collection404()
        except Exception as e:
            logging.error(e)
            raise self.Collection404()

        if col == None:
            raise self.Collection404()

        payload['id'] = str(uuid.uuid4())

        col.append(payload)

        try:
            with open("{}/{}.yml".format(self.source, collection), "w") as file:
                yaml.dump({collection: col, collection + "_count": len(col)}, file)
        except Exception as e:
            raise self.WriteErr()

        return payload['id']
    

    class DBErr(Exception):
        def __init__(self):
            self.message = "server error"
            self.status = 500


    class Collection404(Exception):
        def __init__(self):
            self.message = "couldn't find collection"
            self.status = 404

    class CollectionCreateErr(Exception):
        def __init__(self):
            self.message = "couldn't create collection"
            self.status = 500

            
    class Item404(Exception):
        def __init__(self):
            self.message = "couldn't find item"
            self.status = 404

    class WriteErr(Exception):
        def __init__(self):
            self.message = "couldn't write to file"
            self.status = 500


db = DatabaseService()


class Printer(BaseModel):
    id: str
    port: int

class PrinterCreate(BaseModel):
    port: Optional[int] = None


@app.get("/")
async def root():
   return {"message": "Hello World"}

@app.get("/printers")
async def get_printers():
    printers = try_get_printers()
    
    if not printers:
        return {"message": "Couldn't get printers"}

    return {"printers": printers}


@app.post("/printers")
async def create_printer(item: Optional[PrinterCreate]):

    printers = try_get_printers()

    if printers == None:
        return {"message": "need to first create the collection"}

    def port_taken(printers, port):
        return next((x for x in printers if x['port'] == port), False)

    if item and item.port and port_taken(printers, item.port):
        return {"message": "port is currently taken"}

    if not item or not item.port:
        print('running')
        port = 3000

        while port_taken(printers, port):
            port += 1

        item = PrinterCreate()

        item.port = port

    try:
        popen = subprocess.Popen([
            "./OctoPrint/venv/bin/activate", "-c",
            "octoprint", "serve", "--port='{}'".format(item.port)
        ], stdout=subprocess.PIPE, shell=True)
        popen.wait()
        output = popen.stdout.read()
        print(item)
        printer_id = db.add_to_collection("printers", vars(item))
    except Exception as e:
        logging.error(e)
        return {"message": "Couldn't create printer"}

        
    return {"message": "Printer created: {}".format(printer_id)}


@app.get("/printers/{id}")
async def get_printer(id: int):
    logging.debug("printer: {}".format(id))
    printer = try_get_printer(id)

    if not printer:
        return {"message": "Couldn't find printer"}

    return {"printer": printer}


@app.get("/printers/count")
async def get_printer_count():
    count = try_get_printer_count()

    if count == None:
        return {"message": "Couldn't get printer count"}

    return {"count": count}


@app.put("/printers/{id}")
async def update_printer(id: int):
    printer = try_get_printer(id)

    if not printer:
        return {"message": "No printer with that id"}

    return {"message": "Printer updated"}


def try_get_printers():
    try:
        return db.get_collection('printers')
    except db.Collection404 as e:
        logging.error(e.message)
    except Exception as e:
        logging.error(e)

    return None


def try_get_printer(id: int):
    try:
        return db.get_by_id('printers', id)
    except db.Collection404 as e:
        logging.error(e.message)
    except db.Item404 as e:
        logging.error(e.message)
    except Exception as e:
        logging.error(e)

    return None


def try_get_printer_count():
    try:
        return db.get_collection_count('printers')
    except db.Collection404 as e:
        logging.error(e.message)
    except Exception as e:
        logging.error(e)

    return None
