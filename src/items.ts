class item{
   name: string;
   constructor(name: string){
      this.name = name;
   }
}

//search class inherits from item
export class search extends item{
   query: string;
   constructor(name: string, query: string){
      super(name);
      this.query = query;
   }
}